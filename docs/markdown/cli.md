# CLI

[はじめに](./README.md) / [セットアップ](./getting-started.md) / [対応ブロック一覧](./blocks.md) / [アーキテクチャ](./architecture.md)

実行ファイル名は `xyo` です。開発中は `cargo run -- ...` から呼び出すのが簡単です。

```bash
cargo run -- --help
```

出力:

```
Usage: xyo <COMMAND>

Commands:
  json   project.json を表示する
  stats  統計情報を表示する
  run    解析と IR 生成を実行する
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

現時点のサブコマンドは 3 つです。

## `json`

```bash
cargo run -- json <path-to-project.sb3>
xyo json <path-to-project.sb3>
```

`.sb3` に含まれる `project.json` を標準出力へ表示します。

### 使いどき

- 入力ファイルが正しく開けるか確かめたいとき
- Scratch の生データ構造を確認したいとき
- パーサー調査前に元 JSON を見たいとき
- `project.json` の内容を別のツールに渡したいとき

### 出力例

```json
{
  "targets": [
    {
      "isStage": true,
      "name": "Stage",
      "variables": {},
      "lists": {},
      "broadcasts": {},
      "blocks": {},
      "comments": {},
      "currentCostume": 0,
      "costumes": [...],
      "sounds": [],
      "volume": 100,
      "layerOrder": 0,
      "tempo": 60,
      "videoTransparency": 50,
      "videoState": "on",
      "textToSpeechLanguage": null
    },
    {
      "isStage": false,
      "name": "Sprite1",
      "variables": {},
      "lists": {},
      "broadcasts": {},
      "blocks": {
        "blockId_1": {
          "opcode": "event_whenflagclicked",
          "next": "blockId_2",
          "parent": null,
          "inputs": {},
          "fields": {},
          "shadow": false,
          "topLevel": true
        },
        "blockId_2": {
          "opcode": "motion_movesteps",
          "next": null,
          "parent": "blockId_1",
          "inputs": {
            "STEPS": [1, [4, "10"]]
          },
          ...
        }
      },
      ...
    }
  ],
  "monitors": [],
  "extensions": [],
  "meta": {
    "semver": "3.0.0",
    "vm": "2.3.0",
    "agent": "..."
  }
}
```

### 活用例

`jq` と組み合わせてブロック一覧を抽出することができます。

```bash
# すべてのブロックの opcode を一覧表示する
cargo run -- json my_project.sb3 | jq '[.targets[].blocks[].opcode] | unique | sort'

# ステージ以外のターゲット名を表示する
cargo run -- json my_project.sb3 | jq '[.targets[] | select(.isStage == false) | .name]'

# ブロック数を数える
cargo run -- json my_project.sb3 | jq '[.targets[].blocks | keys | length] | add'
```

## `stats`

```bash
cargo run -- stats <path-to-project.sb3>
xyo stats <path-to-project.sb3>
```

| 項目             | 内容                               |
| ---------------- | ---------------------------------- |
| ファイル名       | 入力した SB3 ファイルのパス        |
| 読み込み時間     | プロジェクト読み込みにかかった時間 |
| ブロック数       | Scratch ブロックの総数             |
| 使用 opcode 一覧 | 含まれている命令種別の配列         |

### 出力例

```
File: my_project.sb3
Loading Time: 2.345ms
Block Number: 42
Using Op Codes: ["event_whenflagclicked", "motion_movesteps", "motion_turnright", "looks_say", "control_repeat"]
```

### 使いどき

- プロジェクトの規模をざっと確認したいとき
- どの opcode が使われているか確認したいとき（`run` 実行前のチェックに便利）
- `run` がどの opcode で失敗するか事前に予想したいとき

### `stats` と対応ブロック一覧の組み合わせ

`stats` で表示された opcode 一覧を [対応ブロック一覧](./blocks.md) と照らし合わせることで、`run` コマンドが最後まで通るかどうかを事前に判断できます。

```
Using Op Codes: ["event_whenflagclicked", "motion_setx", "operator_add"]
```

上記の場合、使われている opcode はすべて IR 生成まで対応しているため `run` が成功する可能性が高いです。

一方、次のような opcode が含まれている場合は `run` が途中で停止することがあります。

```
Using Op Codes: ["event_whenflagclicked", "looks_sayforsecs", "control_if"]
```

`looks_sayforsecs` や `control_if` は現在パーサーまでは対応していますが、IR 生成には未対応です。

## `run`

```bash
cargo run -- run <path-to-project.sb3>
xyo run <path-to-project.sb3>
```

内部では次の順に処理を実行します。

```
1. SB3 ロード      (.sb3 を開き project.json を読み込む)
2. デシリアライズ   (JSON → ScratchProject 構造体)
3. パース          (hat block → Thread, Stmt, Expr)
4. IR 生成         (Thread → LLVM IR 関数)
5. 最適化          (O3 パスを適用)
6. IR 出力         (標準出力へ LLVM IR テキストを表示)
```

```warn
`run` は実行ランタイムではなく、コンパイル経路の検証コマンドです。未実装の opcode や IR 変換が残っているため、入力によっては途中で停止します。
```

### 出力例

成功時は LLVM IR テキストが標準出力に表示されます。最初にターゲット CPU と対応機能の情報が出力された後、LLVM IR が続きます。

```
cpu: x86-64, features: +cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87
; ModuleID = 'xyo'
source_filename = "xyo"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@sprite_0_x = internal global double 0.0
@sprite_0_y = internal global double 0.0
@sprite_0_direction = internal global double 0.0

define void @thread_0(ptr %0) {
entry:
  store double 1.000000e+02, ptr @sprite_0_x, align 8
  ret void
}
```

各部分の意味:

| セクション | 説明 |
| ---------- | ---- |
| `cpu: ...` | ターゲット CPU とその対応機能フラグ |
| `@sprite_N_x` etc. | スプライトの X 座標・Y 座標・向きを表すグローバル変数 |
| `define void @thread_N` | hat block を起点とする各スレッドの関数定義 |
| 関数本体 | スレッドの各命令を LLVM IR に変換した結果 |

### 最適化について

`run` は `default<O3>` 最適化パスを適用します。次のオプションが有効です。

| 最適化 | 説明 |
| ------ | ---- |
| ループインターリービング | ループの実行順序を最適化 |
| ループベクトル化 | SIMD 命令を使って複数の計算を同時処理 |
| SLP ベクトル化 | 隣接するスカラー演算をベクトル演算にまとめる |
| ループアンロール | ループを展開して分岐オーバーヘッドを削減 |

### IR をファイルに保存する

出力をリダイレクトしてファイルに保存できます。

```bash
cargo run -- run my_project.sb3 > output.ll
```

保存した IR は LLVM ツール群で使えます。

```bash
# IR を読みやすくフォーマット
llvm-as output.ll -o output.bc
llvm-dis output.bc -o output_formatted.ll

# ネイティブオブジェクトファイルに変換
llc output.ll -o output.s
```

## エラー表示

読み込みや JSON パースに失敗した場合、原因チェーンをたどって詳細が標準エラー出力に表示されます。失敗箇所によっては JSON のパス、行番号、列番号、周辺コンテキストも確認できます。

### エラーの種類と意味

#### ファイルアクセスエラー

```
Load error: Failed to open SB3 file: `project.sb3`
  cause: No such file or directory (os error 2)
```

指定したパスにファイルが存在しません。パスを確認してください。

#### ZIP 読み込みエラー

```
Load error: Failed to read as SB3 (ZIP): `project.sb3`
  cause: ...
```

ファイルが有効な ZIP アーカイブではありません。ファイルが破損しているか、`.sb3` 形式ではない可能性があります。

#### `project.json` 不在エラー

```
Load error: `project.json` not found in archive: `project.sb3`
```

ZIP アーカイブ内に `project.json` が見つかりません。有効な `.sb3` ファイルか確認してください。

#### JSON パースエラー

```
Load error: Failed to parse `project.json`: `project.sb3`
Path: .targets[0].blocks[blockId_xxx]
Location: line 42, column 15
Context:
  41 | "inputs": {
  42 |   "NUM": [1, "invalid
                  ^
```

- **`Path:`**: パースが失敗した JSON パス。どのブロックで問題が起きたかを示します
- **`Location:`**: `project.json` 内の正確な行・列番号
- **`Context:`**: 周辺テキストとエラー位置を示す `^` 記号

#### パースエラー (未実装 opcode)

```
Parse error: invalid opcode: looks_sayforsecs: target[0].blocks[blockId_xxx]
```

プロジェクトに含まれる opcode がまだパーサーに実装されていません。[対応ブロック一覧](./blocks.md) で対応状況を確認してください。

## 使い分けの目安

| やりたいこと | 推奨コマンド |
| ------------ | ------------ |
| ファイルが正しく読めるか確認 | `json` |
| どんなブロックが使われているか確認 | `stats` |
| IR 生成できるか確認 | `stats` で opcode を確認後 → `run` |
| Scratch の内部構造を調べる | `json` + `jq` |
| コンパイルパイプラインをデバッグ | `run` |

次は [アーキテクチャ](./architecture.md) を参照してください。
