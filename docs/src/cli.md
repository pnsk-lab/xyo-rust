# CLI

`xyo-rust` には現在 3 つのサブコマンドがあります。

## `run`

```bash
cargo run -- run <path-to-sb3>
```

- `.sb3` を読み込む
- プロジェクトを解析する
- スレッドから LLVM IR 生成を試す

現状では実行ランタイムの完成前なので、主用途はコンパイル経路の検証です。

## `stats`

```bash
cargo run -- stats <path-to-sb3>
```

表示内容:

- ファイル名
- 読み込み時間
- ブロック数
- 使用 opcode 一覧

## `json`

```bash
cargo run -- json <path-to-sb3>
```

`.sb3` 内の `project.json` を標準出力へ表示します。

## エラー表示

読み込みや JSON パースに失敗した場合、原因チェーンをたどって詳細が標準エラー出力に表示されます。`project.json` の解析失敗時には、可能な範囲で JSON 上の位置と周辺コンテキストも出力されます。
