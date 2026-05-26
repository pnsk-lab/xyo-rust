# アーキテクチャ

[はじめに](./README.md) / [セットアップ](./getting-started.md) / [CLI](./cli.md) / [対応ブロック一覧](./blocks.md)

`xyo-rust` の処理は、大きく分けると 5 段階です。

1. `.sb3` から `project.json` を取り出す
2. Scratch プロジェクト全体を Rust の構造体へ変換する
3. hat block を起点にスレッド単位の中間表現へ変換する
4. 一部のスレッドを LLVM IR へ変換する
5. 生成した IR を JIT で実行し、状態を定期表示する

Scratch VM 相当の完全な実行ランタイムはまだ完成していないため、現在の中心は解析・変換・JIT 実行のパイプラインです。

## モジュール概要

| モジュール      | 役割                                                                                                        |
| --------------- | ----------------------------------------------------------------------------------------------------------- |
| `src/sb3.rs`    | ZIP アーカイブとして `.sb3` を開き、`project.json` を読み込む。失敗時には位置情報付きのエラー整形も担当する |
| `src/types/`    | Scratch の JSON 構造を受ける型定義を提供する                                                                |
| `src/parser/`   | Scratch ブロックを `Stmt` / `Expr` に変換し、hat block からスレッド単位に解析する                           |
| `src/compiler/` | `inkwell` を使って LLVM IR を生成し、最適化パスと JIT 実行を担当する                                           |

## データフロー詳細

### 全体フロー

```
.sb3 ファイル (ZIP アーカイブ)
        │
        │ File::open + ZipArchive
        ▼
   project.json (バイト列)
        │
        │ serde_path_to_error::deserialize
        ▼
   ScratchProject (Rust 構造体)
   ├── targets: Vec<StageOrSprite>
   │   ├── isStage: bool
   │   ├── name: String
   │   ├── blocks: HashMap<String, BlockAndTopLevelPrimitive>
   │   ├── variables: HashMap<String, Variable>
   │   └── lists: HashMap<String, List>
   └── meta: Metadata
        │
        │ project_parser()
        ▼
   Vec<Thread>
   ├── Thread { hat: HatStmt, stmts: Vec<Stmt>, target_idx: usize }
   ├── Thread { hat: HatStmt, stmts: Vec<Stmt>, target_idx: usize }
   └── ...
        │
        │ compiler()
        ▼
   LLVM Module
   ├── 実行状態 (SpriteStruct / JitSpriteState)
   ├── 文字列リテラル (StringStruct)
   ├── 関数 @thread_0(ptr) → スレッドの IR
   ├── 関数 @thread_1(ptr) → スレッドの IR
   └── ...
        │
        │ run_passes("default<O3>")
        ▼
   JIT 実行
   ├── thread_0(ptr state) を呼ぶ
   └── 変更後の状態を標準出力へ表示
```

## ステージ 1: SB3 ロード (`src/sb3.rs`)

`.sb3` ファイルは ZIP アーカイブです。`sb3.rs` はこのアーカイブを開いて `project.json` を取り出す処理を担当します。

### 主要な公開 API

```rust
// project.json を文字列として返す
pub fn read_json(path: impl AsRef<Path>) -> Result<String, ReadSb3Error>

// project.json をデシリアライズして ScratchProject を返す
pub fn read_sb3(path: impl AsRef<Path>) -> Result<ScratchProject, ReadSb3Error>
```

### 処理の流れ

1. `File::open(path)` でファイルを開く
2. `ZipArchive::new(stream)` で ZIP として読み込む
3. `archive.by_name("project.json")` でエントリを取得する
4. バイト列として読み込む
5. `read_sb3` の場合は `serde_path_to_error::deserialize` でデシリアライズする

### エラー報告の仕組み

`sb3.rs` は JSON パースエラーが発生した際に、単純なエラーメッセージではなく詳細なコンテキスト情報を生成します。

```rust
pub enum ReadSb3Error {
    OpenFile { path: String, source: std::io::Error },
    OpenZip { path: String, source: zip::result::ZipError },
    MissingProjectJson { path: String, source: zip::result::ZipError },
    ReadProjectJson { path: String, source: std::io::Error },
    ParseProjectJson {
        path: String,
        source: serde_json::Error,
        json_path: Option<String>,  // JSON パス (例: .targets[0].blocks[xxx])
        context: String,            // 行番号・列番号・周辺テキスト
    },
    ReadAsUTF8 { path: String, source: FromUtf8Error },
}
```

`serde_path_to_error` クレートを使うことで、デシリアライズが失敗した JSON パスを正確に特定できます。さらに `refine_json_path` 関数によって、エラーの起きたブロック ID までパスを絞り込む場合があります。

エラー表示には `json_error_context` 関数が使われ、行番号・列番号とともに前後 1 行の周辺テキスト（長い行は `...` で省略）と `^` カーソルが表示されます。

## ステージ 2: 型定義とデシリアライズ (`src/types/`)

`types/` モジュールは Scratch の `project.json` 構造を Rust の型として表現します。

### 主要な型

#### `ScratchProject`

プロジェクト全体を表すルート構造体です。

```rust
pub struct ScratchProject {
    pub targets: Vec<StageOrSprite>,
    pub meta: Metadata,
    // ...
}
```

`count_blocks()` メソッドでブロック総数を、`check_op_codes()` メソッドで使用 opcode 一覧を取得できます。

#### `StageOrSprite`

ステージとスプライトを統合した型です。`isStage` フィールドで区別します。

```rust
pub struct StageOrSprite {
    pub is_stage: bool,
    pub name: String,
    pub blocks: HashMap<String, BlockAndTopLevelPrimitive>,
    pub variables: HashMap<String, (String, serde_json::Value)>,
    pub lists: HashMap<String, (String, Vec<serde_json::Value>)>,
    // ...
}
```

#### `Block`

個々のブロックを表す型です。

```rust
pub struct Block {
    pub opcode: BlockOpCodes,
    pub inputs: HashMap<String, Input>,
    pub fields: HashMap<String, Vec<Option<String>>>,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub shadow: bool,
    pub top_level: bool,
}
```

- `opcode`: `BlockOpCodes` 列挙型（`str_enum!` マクロで定義）
- `inputs`: 入力値の辞書（数値・文字列・他ブロックへの参照など）
- `fields`: ドロップダウンメニューの選択値
- `next` / `parent`: ブロック連鎖のリンク

#### `BlockAndTopLevelPrimitive`

ブロック辞書の値型です。通常のブロックと、ブロック参照なしに直接埋め込まれるプリミティブ値の両方を表します。

```rust
pub enum BlockAndTopLevelPrimitive {
    Block(Block),
    Primitive(TopLevelPrimitive),
}
```

#### `Input`

ブロックの入力値を表す型です。Scratch は入力形式として V2 互換形式と V3 形式の両方をサポートしており、この型でその違いを吸収します。

### カスタムマクロ

`types/mod.rs` には型定義を簡潔に書くためのカスタムマクロが定義されています。

#### `str_enum!`

文字列と列挙型を双方向に変換できる列挙型を定義します。Scratch の JSON には opcode などが文字列で格納されているため、デシリアライズ時にこのマクロが活用されています。

```rust
str_enum! {
    pub enum BlockOpCodes {
        MotionMoveSteps => "motion_movesteps",
        MotionTurnRight => "motion_turnright",
        // ...
    }
}
```

#### `str_enum_with_enum!`

カテゴリ情報も付属した列挙型を定義します。

## ステージ 3: パーサー (`src/parser/`)

パーサーは `ScratchProject` を受け取り、各スレッドを表す `Vec<Thread>` を返します。

### スレッドとは

Scratch では、hat block（「緑の旗が押されたとき」など）がスクリプトの起点になります。hat block に続くブロック列が一つの実行単位（スレッド）です。

`Thread` 構造体は次のフィールドを持ちます。

```rust
pub struct Thread {
    pub hat: HatStmt,       // スレッドを起動するイベント
    pub stmts: Vec<Stmt>,   // 実行するブロック列
    pub target_idx: usize,  // どのスプライト/ステージのスレッドか
}
```

### hat block の種類

`HatStmt` 列挙型は 10 種類の hat block を表します。

| バリアント | Scratch での表示 | opcode |
| ---------- | ---------------- | ------ |
| `WhenFlagClicked` | 緑の旗が押されたとき | `EventWhenFlagClicked` |
| `WhenKeyPressed { key }` | `[キー]` キーが押されたとき | `EventWhenKeyPressed` |
| `WhenThisSpriteClicked` | このスプライトが押されたとき | `EventWhenThisSpriteClicked` |
| `WhenStageClicked` | ステージが押されたとき | `EventWhenStageClicked` |
| `WhenBacdropSwitchesTo { backdrop }` | 背景が `[名前]` に切り替わったとき | `EventWhenBackdropSwitchesTo` |
| `WhenGreaterThan { target, value }` | `[音量/タイマー]` が `[値]` より大きくなったとき | `EventWhenGreaterThan` |
| `WhenBroadcastReceived { target }` | `[メッセージ]` を受け取ったとき | `EventWhenBroadcastReceived` |
| `ControlStartAsClone` | クローンされたとき | `ControlStartAsClone` |
| `ProcedureDefinition { prototype }` | 手続き定義（独自ブロック） | `ProceduresDefinition` |
| `WhenTouchingObject { object }` | `[対象]` に触れたとき | `EventWhenTouchingObject` |

### パースの流れ

`project_parser` 関数は次の手順でスレッドを抽出します。

1. すべてのターゲット（スプライト・ステージ）をループで処理する
2. 各ターゲットのすべてのブロックをループで処理する
3. `top_level: true` かつ hat block の opcode を持つブロックを見つける
4. hat block の種別を `hat.rs` で判定して `HatStmt` に変換する
5. `next` フィールドをたどって続くブロック列を解析し、`Vec<Stmt>` を作る
6. `Thread { hat, stmts, target_idx }` を生成して結果リストに追加する

### AST 型

パーサーが生成する AST は `parser/types.rs` で定義されています。

#### `Stmt` (文ブロック)

```rust
pub enum Stmt {
    Motion(MotionStmt),       // 動き
    Looks(LooksStmt),         // 見た目
    Sound(SoundStmt),         // 音
    Event(EventStmt),         // イベント
    Control(ControlStmt),     // 制御
    Sensing(SensingStmt),     // 調べる
    Operator(OperatorStmt),   // 演算 (文はほぼない)
    DataStmt(DataStmt),       // データ
    Procedures(ProceduresStmt), // 独自ブロック
    PenStmt(PenStmt),         // ペン
}
```

各カテゴリは対応する Scratch ブロックのすべての入力・フィールドを保持します。例えば `MotionStmt::GlideToXY` は秒数・X 座標・Y 座標を `Expr` として保持します。

#### `Expr` (値ブロック / レポーター)

```rust
pub enum Expr {
    Motion(MotionExpr),
    Looks(LooksExpr),
    Sound(SoundExpr),
    Event(EventExpr),
    Control(ControlExpr),
    Sensing(SensingExpr),
    Operator(OperatorExpr),
    Data(DataExpr),
    Procedures(ProceduresExpr),
    Pen(PenExpr),
    Literal(Literal),  // 数値・文字列・変数参照などのリテラル
}
```

`Expr` はネストできます。例えば「`(10 + x座標)` 歩動かす」は次のように表現されます。

```
MotionStmt::MoveStep {
    steps: Expr::Operator(OperatorExpr::Add {
        left: Box::new(Expr::Literal(Literal::Number("10".to_string()))),
        right: Box::new(Expr::Motion(MotionExpr::XPosition)),
    })
}
```

#### `Literal` (入力プリミティブ)

`Literal` はブロック以外の生の入力値を表します。

```rust
pub enum Literal {
    String(String),              // テキスト入力
    Number(String),              // 数値入力（文字列として保持）
    Variable { target: String }, // 変数参照 (変数 ID)
    List { target: String },     // リスト参照 (リスト ID)
    Color { color: String },     // 色 (#RRGGBB 形式)
    Broadcast { id: String },    // ブロードキャスト参照
    Null,                        // 空入力
}
```

数値は `String` として保持されており、IR 生成時に `f64` へ変換されます。これは Scratch の数値が浮動小数点演算を前提としているためです。

### カテゴリ別パーサー

`parser/blocks/` ディレクトリには、カテゴリごとのパーサーが 11 ファイルあります。

| ファイル | 担当カテゴリ | 対応ブロック数（概算） |
| -------- | ------------ | ---------------------- |
| `motion.rs` | 動き | 17 Stmt + 8 Expr |
| `looks.rs` | 見た目 | 21 Stmt + 5 Expr |
| `sound.rs` | 音 | 8 Stmt + 4 Expr |
| `event.rs` | イベント | 2 Stmt |
| `control.rs` | 制御 | 15 Stmt + 2 Expr |
| `sensing.rs` | 調べる | 3 Stmt + 22 Expr |
| `operator.rs` | 演算 | 18 Expr |
| `data.rs` | データ | 11 Stmt + 6 Expr |
| `procedures.rs` | 独自ブロック | 1 Stmt + 3 Expr |
| `pen.rs` | ペン | 13 Stmt + 1 Expr |

### エラー処理

`ParserError` 列挙型でパースエラーを表現します。

```rust
pub enum ParserError<'a> {
    NotHandledOp(BlockOpCodes),       // 未実装 opcode
    InvalidValue(&'a str),            // 不正な値
    UnknownBlock(String),             // 未知のブロック ID
    InvalidTargetIndex(usize),        // 不正なターゲットインデックス
    UnexpectedTopLevelPrimitive(String), // 予期しないプリミティブ
    Context {
        context: String,              // コンテキスト情報（ブロック ID など）
        source: Box<ParserError<'a>>, // 原因エラー
    },
}
```

`Context` バリアントにより、エラーが発生したブロック ID やターゲットインデックスを連鎖的に付加できます。これにより「どのスプライトのどのブロックで失敗したか」が分かります。

## ステージ 4: IR 生成と JIT 実行 (`src/compiler/`)

コンパイラは `Vec<Thread>` を受け取り、各スレッドを LLVM IR の関数として生成したあと、JIT で実行して状態を定期表示します。CLI から `run --emit-llvm <path>` を指定した場合は、JIT 実行前にモジュール全体の LLVM IR を指定パスへ保存します。

### `Builders` 構造体

`compiler/types.rs` では、LLVM の基本オブジェクトに加えて、実行時 ABI や変数割り当てをまとめた `Builders` 構造体が定義されています。

`Builders` が保持する主な情報は次のとおりです。

- `context` / `module` / `builder`: LLVM の基本オブジェクト
- `functions`: `llvm.*` intrinsic と `xyo_atod`, `xyo_dtoa`, `xyo_bool_to_str`, `xyo_str_cmp_gt`, `xyo_str_cmp_lt`, `xyo_str_cmp_eq`, `str_to_bool`, `str_is_num`, `xorshift128plus` の関数ハンドル
- `global_variables` / `local_variables`: Scratch の変数 ID をスロット番号へ割り当てる表
- `get_variable()`: 変数 ID を `VariableInfo` に解決して、グローバルかローカルかとスロット番号を返す
- `rolling_hash_seed_*` / `rolling_hash_base_*`: 文字列ハッシュ生成に使う定数
- `string_literals`: 生成済みの `StringStruct` グローバルを再利用するキャッシュ

### 実行時の構造体

```rust
#[repr(C)]
pub struct StringStruct {
    pub length: u64,
    pub container: *mut u16,
    pub hash1: u64,
    pub hash2: u64,
}

#[repr(C)]
pub struct SpriteStruct {
    pub sprite_x: f64,
    pub sprite_y: f64,
    pub sprite_rotate: f64,
}
```

`StringStruct` は UTF-16 の文字列本体と 2 系統のローリングハッシュを持ち、`SpriteStruct` はスプライトの実行状態を表します。`StringKeys` と `SpriteKeys` はこれらの構造体の field index を LLVM から扱いやすくするための列挙型です。

### スレッドごとの IR 生成

各 `Thread` は次のようにして LLVM 関数に変換されます。

```rust
// 関数のシグネチャ: void thread_N(ptr)
let fn_type = context.void_type().fn_type(&[ptr_type.into()], false);
let function = module.add_function(&function_name, fn_type, None);

// エントリブロックを作成してビルダーを配置
let entry = context.append_basic_block(function, "entry");
builder.position_at_end(entry);

// 各 Stmt を IR に変換
for stmt in &thread.stmts {
    match stmt {
        Stmt::Motion(v) => parse_motion_stmt(builders, v, &function, thread.target_idx),
        _ => todo!("やります"),  // 未実装
    }
}

builder.build_return(None);  // void return
```

### 実行状態

各スレッドは `SpriteStruct` と同じ 3 フィールド構成の状態ポインタを受け取り、`MotionSetX` などは `build_struct_gep` を使ってそのフィールドを更新します。JIT 実行時に表示される `JitSpriteState` も、同じレイアウトを持つデバッグ用の状態構造体です。

### 式の IR 変換

`generate_expr_ir` 関数が式を LLVM の値に変換します。戻り値の型は `ScratchReturnTypes` 列挙型で表現されます。

```rust
pub enum ScratchReturnTypes<'ctx> {
    Number(FloatValue<'ctx>),                    // f64 の LLVM 値
    String(PointerValue<'ctx>),                  // 文字列ポインタ
    Bool(IntValue<'ctx>),                        // i1 の LLVM 値（真偽値）
    NumberLiteral(f64),                          // コンパイル時定数（数値）
    StringLiteral((String, PointerValue<'ctx>)), // コンパイル時定数（文字列）
    BoolLiteral((bool, IntValue<'ctx>)),         // コンパイル時定数（真偽値）
}
```

`Literal` バリアントは実際の LLVM 値と Rust 側の定数の両方を保持します。これにより、定数畳み込みなどの最適化が可能になります。`String` / `StringLiteral` は `StringStruct` で表現された文字列を指し、`BoolLiteral` / `NumberLiteral` は coercion のための定数としても使われます。

### 最適化

IR 生成後、`default<O3>` パスが適用されます。有効化されている最適化オプション:

- **ループインターリービング**: ループの実行順序を最適化してキャッシュ効率を上げる
- **ループベクトル化**: SIMD 命令を使って複数の計算を同時処理
- **SLP ベクトル化**: 隣接するスカラー演算をベクトル演算にまとめる
- **ループアンロール**: ループを展開して分岐オーバーヘッドを削減

## ビルドスクリプト (`build.rs`)

`cargo build` 時に `build.rs` が実行され、`bitcodes/c/` にあるトップレベルの C ソースを LLVM bitcode と LLVM IR に変換します。各ソースは `clang -emit-llvm -c -O3` で `.bc`、`clang -S -emit-llvm -O3` で `.ll` に変換され、出力は `bitcodes/bc/` と `bitcodes/ll/` に保存されます。生成済み `.bc` は `embedded_bitcodes.rs` にも列挙されます。

現在のターゲット:

| C ソース | 生成物 | 役割 |
| -------- | ------ | ---- |
| `bitcodes/c/numeric.c` | `bitcodes/bc/numeric.bc`, `bitcodes/ll/numeric.ll` | 数値変換と数値判定のヘルパー |
| `bitcodes/c/str.c` | `bitcodes/bc/str.bc`, `bitcodes/ll/str.ll` | 文字列比較と真偽値変換のヘルパー |
| `bitcodes/c/tick.c` | `bitcodes/bc/tick.bc`, `bitcodes/ll/tick.ll` | 単調時計と sleep のヘルパー |

`numeric.c` は `bitcodes/c/lib/dtoa.c` と `bitcodes/c/lib/cutils.c` を取り込み、`xyo_atod` / `xyo_dtoa` / `str_is_num` / `str_is_double` を実装します。`str.c` は ICU の Unicode API を使って `string_to_bool` と文字列比較を実装し、`tick.c` は `xyo_now_ns` / `xyo_sleep_until_ns` を提供します。

`build.rs` は `CLANG`、`PATH` 上の `clang`、`llvm-config --bindir` の順で `clang` を解決し、`bitcodes/c/lib` と ICU の include path を追加して各ソースをコンパイルします。ICU ヘッダは `XYO_ICU_PREBUILT_DIR/include` か `XYO_ICU_ROOT/source/common` から見つけます。

ビルドスクリプトは次の手順で動作します:

1. `CLANG`、`PATH` 上の `clang`、`llvm-config --bindir` の順で `clang` を解決する
2. `bitcodes/c/` 配下のトップレベル `.c` を列挙する
3. 各ソースに `-I bitcodes/c/lib` と ICU の include path を足して `clang -emit-llvm -c -O3` と `clang -S -emit-llvm -O3` を実行する
4. 生成した `.bc` を `bitcodes/bc/`、`.ll` を `bitcodes/ll/` に保存する
5. `.bc` 一覧をもとに `embedded_bitcodes.rs` を書き出す

## 制約と今後の方針

### 現在の制約

- **IR 生成**: 動き系命令と演算子のみ。`run` で残りの opcode に当たると `todo!()` パニックが起きる
- **ランタイム**: Scratch のイベントループや broadcast / clone を含む完全な VM は未実装。いまの `run` は JIT で各 thread を実行し、状態を標準出力へ定期的に返す
- **コスチューム・サウンド**: メタデータは読み込めるが IR 生成には未対応
- **スレッド間通信**: ブロードキャスト・メッセージ処理は未実装

### 今後の実装が期待される部分

- `ControlStmt` (if/else, repeat, forever など) の IR 生成
- `DataStmt` (変数操作) の IR 生成
- `LooksStmt` (見た目変更) の IR 生成（実際のレンダリングは別ライブラリが必要）
- `run` の未実装分岐に対する安全なフォールバック（パニックを避けてエラー報告する）
- 生成した IR を `clang` や `llc` でリンク・コンパイルするフロー

前のページ: [CLI](./cli.md)
