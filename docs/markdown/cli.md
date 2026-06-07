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
  run      run <path>
  compile  compile <path>
  stats    stats <path>
  json     json <path>
  help     Print this message or the help of the given subcommand(s)

Options:
  -V, --version  Print version
  -h, --help  Print help
```

現時点のサブコマンドは 4 つです。

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

上記の場合、文ブロックは動き系で、入力式も演算子だけなので `run` が成功する可能性が高いです。

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
6. JIT 実行         (生成した各 thread 関数を起動し、実行中の状態を表示する)
7. 状態表示        (各 thread の状態スナップショットを Debug 形式で表示)
```

```warn
`run` は実行ランタイムそのものではなく、コンパイル経路を JIT までつないだ検証コマンドです。未実装の opcode や IR 変換が残っているため、入力によっては途中で停止します。
```

### 出力例

成功時は各スレッドの状態が実行中に標準出力へ表示されます。

```
SpriteStruct { sprite_x: 100.0, sprite_y: 0.0, sprite_rotate: 90.0, sprite_size: 100.0, ... }
```
<!--
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"


define void @thread_0(ptr %0) {
entry:
  store double 1.000000e+02, ptr @sprite_0_x, align 8
  ret void
}
```
-->

各部分の意味:

| セクション | 説明 |
| ---------- | ---- |
| `SpriteStruct` | 実行中に表示されるスプライト状態 |
| `sprite_x` | スプライトの X 座標 |
| `sprite_y` | スプライトの Y 座標 |
| `sprite_rotate` | スプライトの向き |
| `sprite_size` | スプライトの大きさ |

### 最適化について

`run` は `default<O3>` 最適化パスを適用してから JIT 実行します。次のオプションが有効です。

| 最適化 | 説明 |
| ------ | ---- |
| ループインターリービング | ループの実行順序を最適化 |
| ループベクトル化 | SIMD 命令を使って複数の計算を同時処理 |
| SLP ベクトル化 | 隣接するスカラー演算をベクトル演算にまとめる |
| ループアンロール | ループを展開して分岐オーバーヘッドを削減 |

### 補足

`run` の標準出力には、既定では各 thread の状態スナップショットが表示されます。LLVM IR テキストを確認したい場合は `compile` を使ってください。

## `compile`

```bash
cargo run -- compile <path-to-project.sb3> --output out.ll
xyo compile <path-to-project.sb3> --output out.ll
```

`.sb3` を読み込み、hat block から thread を抽出して LLVM IR を生成し、指定した `.ll` ファイルへ保存します。JIT 実行は行いません。

`--output` を省略した場合は `out.ll` に保存します。

```bash
cargo run -- compile <path-to-project.sb3>
xyo compile <path-to-project.sb3>
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
| JIT 実行できるか確認 | `stats` で opcode を確認後 → `run` |
| 生成された LLVM IR を確認 | `compile --output out.ll` |
| Scratch の内部構造を調べる | `json` + `jq` |
| コンパイルパイプラインをデバッグ | `compile` |

次は [アーキテクチャ](./architecture.md) を参照してください。
