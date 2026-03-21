# xyo-rust

`xyo-rust` は、Scratch の `.sb3` プロジェクトを Rust で読み込み、構文解析し、LLVM IR 生成へ接続する実験的なランタイム / コンパイラ基盤です。

いまの主眼は **SB3 ローダー・パーサー・IR 生成経路の検証** にあります。Scratch VM と同等の完全実行を目指す段階ではなく、まずは「Scratch プロジェクトをどこまで静的に扱えるか」を試すための土台が実装されています。

## Scratch と `.sb3` について

[Scratch](https://scratch.mit.edu/) は MIT メディアラボが開発したビジュアルプログラミング環境です。`.sb3` ファイルは ZIP アーカイブで、内部の `project.json` にすべてのブロック・スプライト・変数などのメタデータが JSON 形式で格納されています。

`xyo-rust` はこの `project.json` を Rust の型として読み込み、hat block（「緑の旗が押されたとき」など）を起点にスクリプトを解析して LLVM IR へ変換します。

## 処理パイプライン

```
.sb3 ファイル
      │
      ▼
[SB3 ロード]  src/sb3.rs
  ZIP を展開し project.json を取り出す
      │
      ▼
[デシリアライズ]  src/types/
  project.json → ScratchProject (Rust 構造体)
      │
      ▼
[パース]  src/parser/
  hat block → Thread (スレッド)
  各ブロック → Stmt / Expr (AST)
      │
      ▼
[IR 生成]  src/compiler/
  Thread → LLVM 関数
  最適化パス (O3) を適用
      │
      ▼
LLVM IR テキスト (標準出力)
```

## 現在できること

| 機能 | コマンド | 状態 |
| ---- | -------- | ---- |
| `project.json` を取り出して表示する | `json` | ✅ |
| ブロック数・使用 opcode を確認する | `stats` | ✅ |
| hat block からスレッドを抽出する | `run` | ✅ |
| 動き系命令・演算子を LLVM IR へ変換する | `run` | 🚧 一部 |
| JSON パースエラー時の位置情報・コンテキスト表示 | — | ✅ |
| 完全な Scratch 互換実行 | — | ❌ |

## まだ開発途中のこと

- Scratch opcode の網羅的な IR 実装（現状は動き系 + 演算子のみ）
- 実行ランタイムの完成
- 生成した IR から実行可能ファイルへつなぐフロー
- 互換性検証とリグレッションテストの拡充
- `run` サブコマンドの未実装命令に対する安全なフォールバック

## 必要環境

| ツール | バージョン | 確認コマンド |
| ------ | ---------- | ------------ |
| Rust | stable | `rustc --version` |
| LLVM | 21.1.x | `llvm-config --version` |
| `llvm-config` | PATH 上にあること | `which llvm-config` |
| `clang` | PATH 上にあること | `which clang` |

`inkwell` を使っているため、LLVM のメジャー・マイナー差異には注意が必要です。作業前に `llvm-config --version` が `21.1.x` を返すことを確認してください。

## クイックスタート

### ビルド

```bash
git clone https://github.com/pnsk-lab/xyo-rust.git
cd xyo-rust
cargo build --release
```

`cargo build` は `build.rs` を通じて `bitcodes/` 配下の C コードも再生成します。ビルドが成功すると `target/release/xyo` が生成されます。

### テスト

```bash
cargo test
```

### CLI ヘルプ

```bash
cargo run -- --help
```

## CLI

生成される実行ファイル名は `xyo` です。開発中は `cargo run -- ...` で試せます。

### 統計情報を見る

```bash
cargo run -- stats <path-to-project.sb3>
```

出力例:

```
File: my_project.sb3
Loading Time: 2.345ms
Block Number: 42
Using Op Codes: ["event_whenflagclicked", "motion_movesteps", "operator_add"]
```

### `project.json` を表示する

```bash
cargo run -- json <path-to-project.sb3>
```

`.sb3` に含まれる `project.json` をそのまま表示します。`jq` などと組み合わせてフィルタリングできます。

```bash
cargo run -- json my_project.sb3 | jq '[.targets[].blocks[].opcode] | unique | sort'
```

### 解析と IR 生成を試す

```bash
cargo run -- run <path-to-project.sb3>
```

`run` は現状もっとも実験的なコマンドです。動き系命令と演算子のみを含むシンプルなプロジェクトから試すことを推奨します。成功時は LLVM IR テキストが出力されます。

## 入力ファイルについて

このリポジトリには現在、配布用の `.sb3` サンプルは含まれていません。Scratch エディタでプロジェクトを作成し、**「ファイル」→「コンピューターに保存する」** で `.sb3` を書き出して入力に使ってください。

`run` で最後まで通したい場合は、動き系ブロック（「〇歩動かす」「x座標を〇にする」）と演算子のみを使ったシンプルなプロジェクトから始めると確認しやすいです。

## プロジェクト構成

```
xyo-rust/
├── src/
│   ├── main.rs          CLI エントリポイントとエラー出力
│   ├── cli.rs           サブコマンド定義 (clap)
│   ├── sb3.rs           .sb3 / project.json の読み込みと詳細エラー整形
│   ├── types/           Scratch JSON 構造を受ける型定義
│   ├── parser/          Scratch ブロック列を Stmt / Expr に変換
│   └── compiler/        LLVM IR 生成
├── tests/               CLI テスト
├── bitcodes/            C ソースと生成済み bitcode / IR
├── docs/                Markdown ソースと Taiga サイト生成ファイル
├── build.rs             ビルドスクリプト (C → bitcode)
└── Cargo.toml           プロジェクト設定
```

## CI

| ワークフロー | トリガー | 内容 |
| ------------ | -------- | ---- |
| `build.yml` | GitHub Release 公開時 | Rust バイナリのマルチプラットフォームビルド |
| `bitcodes.yml` | push / PR | `bitcodes/` 配下の C ソースから `.bc` / `.ll` を自動生成 |
| `pages.yml` | push to main | ドキュメントを GitHub Pages へデプロイ |

## Bitcodes

`bitcodes/` には LLVM bitcode と LLVM IR の生成元 C コードがあります。`cargo build` 時に `bitcodes/bc/` と `bitcodes/ll/` が更新され、Git には含めません。

ローカルで再生成するには次を使います。

```bash
./bitcodes/build.sh
```

古い出力を消して作り直す場合は `--clean`、全部強制再生成する場合は `--force` を付けます。

## ドキュメント

詳細は [ドキュメントサイト](./docs/markdown/README.md) を参照してください。

| ページ | 内容 |
| ------ | ---- |
| [セットアップ](./docs/markdown/getting-started.md) | LLVM インストール・ビルド手順・最初のコマンド |
| [CLI](./docs/markdown/cli.md) | サブコマンドの詳細・出力例・エラーの読み方 |
| [対応ブロック一覧](./docs/markdown/blocks.md) | opcode ごとのパーサー / IR 対応状況 |
| [アーキテクチャ](./docs/markdown/architecture.md) | パイプラインの詳細・モジュール設計 |

## ライセンス

MIT
