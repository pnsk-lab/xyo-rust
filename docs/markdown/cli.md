# CLI

[はじめに](./README.md) / [セットアップ](./getting-started.md) / [対応ブロック一覧](./blocks.md) / [アーキテクチャ](./architecture.md)

実行ファイル名は `xyo` です。開発中は `cargo run -- ...` から呼び出すのが簡単です。

```bash
cargo run -- --help
```

現時点のサブコマンドは 3 つです。

## `json`

```bash
cargo run -- json <path-to-project.sb3>
```

`.sb3` に含まれる `project.json` を標準出力へ表示します。

- 入力ファイルが正しく開けるか確かめたいとき
- Scratch の生データ構造を確認したいとき
- パーサー調査前に元 JSON を見たいとき

## `stats`

```bash
cargo run -- stats <path-to-project.sb3>
```

| 項目             | 内容                               |
| ---------------- | ---------------------------------- |
| ファイル名       | 入力した SB3 ファイル名            |
| 読み込み時間     | プロジェクト読み込みにかかった時間 |
| ブロック数       | Scratch ブロック数                 |
| 使用 opcode 一覧 | 含まれている命令種別               |

## `run`

```bash
cargo run -- run <path-to-project.sb3>
```

内部では、読み込み、デシリアライズ、hat block からのスレッド抽出、LLVM IR 生成の順に処理します。

```warn
`run` は実行ランタイムではなく、コンパイル経路の検証コマンドです。未実装の opcode や IR 変換が残っているため、入力によっては途中で停止します。
```

## エラー表示

読み込みや JSON パースに失敗した場合、原因チェーンをたどって詳細が標準エラー出力に表示されます。失敗箇所によっては JSON のパス、行番号、列番号、周辺コンテキストも確認できます。

## 使い分けの目安

- まず中身を見たい: `json`
- 対応状況をざっと知りたい: `stats` / [対応ブロック一覧](./blocks.md)
- IR 生成経路を試したい: `run`

次は [アーキテクチャ](./architecture.md) を参照してください。
