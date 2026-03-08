# xyo-rust

`xyo-rust` は、Scratch の `.sb3` プロジェクトを読み込み、構文解析し、LLVM を使ってネイティブ実行につなげることを目指す Rust 製ランタイム / コンパイラです。

現時点では **SB3 の読み込み・JSON 抽出・AST/IR 生成の基盤実装が中心** で、実行系はまだ開発途中です。

## できること

- `.sb3` ファイルから `project.json` を読み込む
- Scratch プロジェクトの統計情報を確認する
- `project.json` をそのまま出力する
- hat block からスレッドを抽出し、内部表現へ変換する
- LLVM IR を生成するための基盤コードを試す

## まだ開発途中のこと

- Scratch 命令の網羅的な実装
- ネイティブ実行ランタイムの完成
- 最適化済みバイナリの生成フロー整備
- エラーレポートと互換性検証の拡充

## 必要環境

- Rust stable
- LLVM 21.1 系
- `llvm-config` が利用可能な環境

`inkwell` を利用しているため、LLVM のバージョン差異には注意してください。CI では LLVM 21.1 系を前提にしています。

## ビルド

```bash
cargo build --release
```

## テスト

```bash
cargo test
```

## CLI

### 統計情報を見る

```bash
cargo run -- stats examples/simple.sb3
```

出力例:

- 読み込み時間
- ブロック数
- 使用されている opcode 一覧

### `project.json` を表示する

```bash
cargo run -- json examples/simple.sb3
```

### 解析と IR 生成を試す

```bash
cargo run -- run examples/simple.sb3
```

## プロジェクト構成

- `src/main.rs`: CLI エントリポイント
- `src/cli.rs`: サブコマンド定義
- `src/sb3.rs`: `.sb3` / `project.json` 読み込み
- `src/parser/`: Scratch ブロック列を内部表現へ変換
- `src/compiler/`: LLVM IR 生成
- `examples/`: 動作確認用の `.sb3` サンプル
- `tests/`: CLI テスト
- `docs/`: GitHub Pages 用ドキュメント

## GitHub Pages ドキュメント

このリポジトリには `mdBook` ベースのドキュメントを追加してあります。GitHub の Pages 設定で **Build and deployment = GitHub Actions** を選ぶと、ドキュメント変更の push 時や GitHub Release の publish 時にドキュメントサイトを自動公開できます。

公開 URL の一般形は次のとおりです。

- `https://<owner>.github.io/<repo>/`

詳しい内容は `docs/` 配下を参照してください。

## CI

- `.github/workflows/build.yml`: GitHub Release 公開時の Rust バイナリのマルチプラットフォームビルド
- `.github/workflows/docs.yml`: ドキュメント変更時の push または GitHub Release 公開時の GitHub Pages 向けドキュメント生成・配備

## ライセンス

MIT
