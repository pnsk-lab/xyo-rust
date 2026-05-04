# xyo-rust

[![CI](https://github.com/pnsk-lab/xyo-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/pnsk-lab/xyo-rust/actions/workflows/ci.yml)

`xyo-rust` は、Scratch の `.sb3` プロジェクトを Rust で読み込み、構文解析し、LLVM IR を生成して JIT で動かす実験的なランタイム / コンパイラ基盤です。

いまの主眼は **SB3 ローダー・パーサー・IR 生成 / JIT 実行経路の検証** にあります。Scratch VM と同等の完全実行を目指す段階ではなく、まずは「Scratch プロジェクトをどこまで静的に扱えるか」を試すための土台が実装されています。

## Scratch と `.sb3` について

[Scratch](https://scratch.mit.edu/) は MIT メディアラボが開発したビジュアルプログラミング環境です。`.sb3` ファイルは ZIP アーカイブで、内部の `project.json` にすべてのブロック・スプライト・変数などのメタデータが JSON 形式で格納されています。

`xyo-rust` はこの `project.json` を Rust の型として読み込み、hat block（「緑の旗が押されたとき」など）を起点にスクリプトを解析して LLVM IR を生成し、JIT で動かします。

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
[IR 生成 + JIT 実行]  src/compiler/
  Thread → LLVM 関数
  最適化パス (O3) を適用
  JIT で各スレッドを実行し、状態を標準出力へ表示
      │
      ▼
実行結果 (標準出力)
```

## 現在できること

| 機能 | コマンド | 状態 |
| ---- | -------- | ---- |
| `project.json` を取り出して表示する | `json` | ✅ |
| ブロック数・使用 opcode を確認する | `stats` | ✅ |
| hat block からスレッドを抽出する | `run` | ✅ |
| 動き系命令・演算子を LLVM IR へ変換し、JIT で実行する | `run` | 🚧 一部 |
| JSON パースエラー時の位置情報・コンテキスト表示 | — | ✅ |
| 完全な Scratch 互換実行 | — | ❌ |

## まだ開発途中のこと

- Scratch opcode の網羅的な IR 実装（現状は動き系 + 演算子のみ）
- Scratch VM 相当のイベントランタイムの完成
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

### 解析と JIT 実行を試す

```bash
cargo run -- run <path-to-project.sb3>
```

`run` は現状もっとも実験的なコマンドです。動き系命令と演算子のみを含むシンプルなプロジェクトから試すことを推奨します。成功時は各スレッドの実行後状態が標準出力に表示されます。

```text
JitSpriteState { sprite_x: 100.0, sprite_y: 0.0, sprite_rotate: 0.0 }
```

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
| `ci.yml` | `push` / `pull_request` / `workflow_dispatch` | Ubuntu 上で `cargo test` を回す通常 CI |
| `build.yml` | GitHub Release 用の tag push | Rust バイナリのマルチプラットフォームビルド |
| `pages.yml` | ドキュメント更新時の push | ドキュメントを GitHub Pages へデプロイ |

## Bitcodes

`bitcodes/` には LLVM bitcode と LLVM IR の生成元 C コードがあります。`bitcodes/c/` 配下のトップレベル `.c` ファイルが `cargo build` 時にまとめて再生成され、対応する出力が `bitcodes/bc/` と `bitcodes/ll/` に書き出されます。Git には生成物を含めません。

現在は `dtoa.c`、`atod.c`、`to_lower.c` が対象です。`to_lower.c` は、`XYO_ICU_ROOT` で指定した ICU source tree、または [`bitcodes/c/lib/icu-prebuilt`](/home/kicky/dev/xyo-rust/bitcodes/c/lib/icu-prebuilt) の `include/` を使って bitcode / IR を生成します。日常運用では、同じ ICU から生成した prebuilt static archive をネイティブ動作確認に使うルートを正式扱いにしています。ICU ソースごと `to_lower.bc` に埋め込む重い自己完結ビルドは、`XYO_EMBED_ICU_BITCODE=1` を付けたときだけ有効です。

セットアップ全体を流すには次を使います。

```bash
CLANG=clang-23 \
CLANGXX=clang++-23 \
./setup.sh
```

`setup.sh` は prebuilt ICU 構築、`cargo build --release`、`to_lower.ll` の native check までを順に実行します。

通常の `cargo build` を軽く保つため、`build.rs` はデフォルトでは ICU ソース全体の bitcode 化を行いません。自己完結な `to_lower.bc` を試したいときだけ、次のように明示 opt-in します。

```bash
XYO_EMBED_ICU_BITCODE=1 cargo build --release
```

ICU source tree が既定位置に無い場合は、`XYO_ICU_ROOT=/path/to/icu` を付けて `setup.sh` や `cargo build` を実行できます。prebuilt archive の配置先を変えたい場合は `XYO_ICU_PREBUILT_DIR=/path/to/prebuilt` を使います。

ローカルで再生成するには次を使います。

```bash
./bitcodes/build.sh
```

古い出力を消して作り直す場合は `--clean`、全部強制再生成する場合は `--force` を付けます。

`to_lower.ll` をネイティブ実行で確認したい場合は、事前に構築した ICU の static archive を使って次を実行できます。

まず vendored ICU から prebuilt archive だけを作る場合は次を使います。

```bash
CLANG=clang-23 \
CLANGXX=clang++-23 \
./tools/build_icu_prebuilt.sh
```

生成先は [`bitcodes/c/lib/icu-prebuilt`](/home/kicky/dev/xyo-rust/bitcodes/c/lib/icu-prebuilt) です。`--clean` を付けると configure 結果から作り直します。

```bash
XYO_ICU_PREBUILT_DIR=/path/to/icu \
CLANG=clang-23 \
CLANGXX=clang++-23 \
./tools/check_to_lower_native.sh
```

このスクリプトは [`tools/to_lower_harness.c`](/home/kicky/dev/xyo-rust/tools/to_lower_harness.c) を使って [`bitcodes/ll/to_lower.ll`](/home/kicky/dev/xyo-rust/bitcodes/ll/to_lower.ll) をネイティブリンクし、`""`, `"0"`, `"false"`, `"FALSE"`, `"true"` などの基本ケースを確認します。
`XYO_ICU_PREBUILT_DIR` を省略した場合は [`bitcodes/c/lib/icu-prebuilt`](/home/kicky/dev/xyo-rust/bitcodes/c/lib/icu-prebuilt) を自動で見に行きます。既存の `XYO_ICU_NATIVE_LIB_DIR` も後方互換として使えます。

## ドキュメント

詳細は [ドキュメントサイト](./docs/markdown/README.md) を参照してください。

| ページ | 内容 |
| ------ | ---- |
| [セットアップ](./docs/markdown/getting-started.md) | LLVM インストール・ビルド手順・最初のコマンド |
| [CLI](./docs/markdown/cli.md) | サブコマンドの詳細・出力例・エラーの読み方 |
| [対応ブロック一覧](./docs/markdown/blocks.md) | opcode ごとのパーサー / IR 対応状況 |
| [アーキテクチャ](./docs/markdown/architecture.md) | パイプラインの詳細・モジュール設計 |

## Star History

<a href="https://www.star-history.com/?repos=pnsk-lab%2Fxyo-rust&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=pnsk-lab/xyo-rust&type=date&theme=dark&legend=top-left" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=pnsk-lab/xyo-rust&type=date&legend=top-left" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=pnsk-lab/xyo-rust&type=date&legend=top-left" />
 </picture>
</a>

## ライセンス

MIT
