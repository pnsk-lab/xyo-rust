# xyo-rust

[![CI](https://github.com/pnsk-lab/xyo-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/pnsk-lab/xyo-rust/actions/workflows/ci.yml)
[![技術者倫理 遵守済み](https://img.shields.io/badge/%E6%8A%80%E8%A1%93%E8%80%85%E5%80%AB%E7%90%86-%E9%81%B5%E5%AE%88%E6%B8%88%E3%81%BF-0a0a0a?style=for-the-badge&labelColor=ffffff)](https://技術者倫理.com)

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
| 動き系命令、見た目の大きさ変更系命令、変数代入、それらの入力式に使われる演算子や変数参照を LLVM IR へ変換し、JIT で実行する | `run` | 🚧 一部 |
| 生成した LLVM IR を `.ll` ファイルへ保存する | `compile` | ✅ |
| JSON パースエラー時の位置情報・コンテキスト表示 | — | ✅ |
| 完全な Scratch 互換実行 | — | ❌ |

## まだ開発途中のこと

- Scratch opcode の網羅的な IR 実装（現状はスレッド本体が動き系、見た目の大きさ変更系、変数代入のみ。式はリテラル、演算子、変数参照、一部レポーターのみ）
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

### Docker イメージ

Docker イメージは `public.ecr.aws/b9q9k6r0/xyo-rust` にあります。

```bash
docker pull public.ecr.aws/b9q9k6r0/xyo-rust
docker run --rm public.ecr.aws/b9q9k6r0/xyo-rust --help
```

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

`run` は現状もっとも実験的なコマンドです。文ブロックは動き系、見た目の大きさ変更系、変数代入、入力式はリテラル・演算子・変数参照・大きさレポーターのみを含むシンプルなプロジェクトから試すことを推奨します。成功時は各スレッドの状態が実行中に定期的に標準出力へ表示されます。

```text
SpriteStruct { sprite_x: 100.0, sprite_y: 0.0, sprite_rotate: 90.0, sprite_size: 100.0, ... }
```

### LLVM IR を生成する

```bash
cargo run -- compile <path-to-project.sb3> --output out.ll
```

`compile` は `.sb3` を LLVM IR に変換して `.ll` ファイルへ保存します。JIT 実行は行いません。

```bash
cargo run -- compile <path-to-project.sb3>
```

`--output` を省略した場合は `out.ll` に保存します。

## 入力ファイルについて

このリポジトリには現在、配布用の `.sb3` サンプルは含まれていません。Scratch エディタでプロジェクトを作成し、**「ファイル」→「コンピューターに保存する」** で `.sb3` を書き出して入力に使ってください。

`run` で最後まで通したい場合は、文ブロックを動き系（「〇歩動かす」「x座標を〇にする」など）、見た目の大きさ変更系（「大きさを〇ずつ変える」「大きさを〇%にする」）、変数代入に絞り、その入力式にリテラル・演算子・変数参照・大きさレポーターだけを使ったシンプルなプロジェクトから始めると確認しやすいです。

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
| `ci.yml` | `push` / `pull_request` / `workflow_dispatch` | Ubuntu 上で `cargo test` と s390x の emulated test を回す通常 CI |
| `build.yml` | GitHub Release 用の tag push | Rust バイナリのマルチプラットフォームビルド |
| `pages.yml` | ドキュメント更新時の push | ドキュメントを GitHub Pages へデプロイ |

## Bitcodes

`bitcodes/` には、`build.rs` で LLVM bitcode と LLVM IR に変換する C ソースとその生成物があります。`bitcodes/c/` 配下のトップレベル `.c` が `cargo build` 時にまとめて再生成され、出力は `bitcodes/bc/` と `bitcodes/ll/` に書き出されます。生成物は Git に含めません。

| C ソース | 生成物 | 役割 |
| ---- | ---- | ---- |
| `bitcodes/c/numeric.c` | `bitcodes/bc/numeric.bc`, `bitcodes/ll/numeric.ll` | 数値変換と数値判定のヘルパー |
| `bitcodes/c/str.c` | `bitcodes/bc/str.bc`, `bitcodes/ll/str.ll` | 文字列比較と真偽値変換のヘルパー。ICU の Unicode API を使う |
| `bitcodes/c/tick.c` | `bitcodes/bc/tick.bc`, `bitcodes/ll/tick.ll` | 単調時計と sleep のヘルパー |

`numeric.c` は `bitcodes/c/lib/dtoa.c` と `bitcodes/c/lib/cutils.c` を取り込み、`xyo_atod` / `xyo_dtoa` / `str_is_num` / `str_is_double` を実装します。`str.c` は ICU の Unicode API を使って `string_to_bool` と文字列比較を実装し、`tick.c` は `xyo_now_ns` / `xyo_sleep_until_ns` を提供します。

`str.c` が ICU ヘッダを使うため、`build.rs` は `XYO_ICU_PREBUILT_DIR/include` か `XYO_ICU_ROOT/source/common` を見ます。`setup.sh` は vendored ICU の取得、prebuilt ICU の構築、`cargo build --release` を順に実行します。

ICU の prebuilt だけを作りたい場合は次を使います。

```bash
CLANG=clang-23 \
CLANGXX=clang++-23 \
./tools/build_icu_prebuilt.sh
```

既定の ICU source tree が `bitcodes/c/lib/icu/` に無い場合は、`XYO_ICU_ROOT=/path/to/icu` を付けて `setup.sh` や `cargo build` を実行できます。prebuilt archive の配置先を変えたい場合は `XYO_ICU_PREBUILT_DIR=/path/to/prebuilt` を使います。

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
