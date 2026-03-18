# xyo-rust ドキュメント

[![Build](https://github.com/pnsk-lab/xyo-rust/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/pnsk-lab/xyo-rust/actions/workflows/build.yml)

[latest release](https://github.com/pnsk-lab/xyo-rust/releases/latest)

Scratch の `.sb3` プロジェクトを読み込み、解析し、LLVM IR 生成へ接続する実験プロジェクト `xyo-rust` の手引きです。

## 目次

- [はじめに](./README.md)
- [セットアップ](./getting-started.md)
- [CLI](./cli.md)
- [対応ブロック一覧](./blocks.md)
- [アーキテクチャ](./architecture.md)

## このプロジェクトが目指すこと

- Scratch プロジェクトを静的に解析できる内部表現へ変換する
- hat block からスレッドを抽出する
- LLVM IR を生成し、将来的なネイティブ実行につなげる
- SB3 解析系の検証用ツールチェーンを整える

## 現在の実装範囲

1. `.sb3` を ZIP として開く
2. `project.json` を取り出す
3. `serde` で Scratch プロジェクト構造へ変換する
4. hat block からスレッドを解析する
5. `Stmt` / `Expr` に変換する
6. 一部の式 / 文を LLVM IR へ変換する

```warn
完全な Scratch 実行互換はまだ未実装です。特に `run` サブコマンドは実験段階で、入力によっては未実装分岐に到達します。
```

## 読み進め方

- まず動かしたい: [セットアップ](./getting-started.md)
- コマンドの違いを知りたい: [CLI](./cli.md)
- 対応済み opcode を確認したい: [対応ブロック一覧](./blocks.md)
- 内部構造を把握したい: [アーキテクチャ](./architecture.md)

## 現時点での注意点

- Scratch VM との完全互換は未実装です
- LLVM 21.1 系を前提にしています
- 配布用の `.sb3` サンプルは同梱されていません

## 関連リンク

- [リポジトリの README](../README.md)
- [GitHub](https://github.com/pnsk-lab/xyo-rust)

続きは [セットアップ](./getting-started.md) を参照してください。
