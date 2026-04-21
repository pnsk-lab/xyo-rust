# xyo-rust ドキュメント

[![CI](https://github.com/pnsk-lab/xyo-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/pnsk-lab/xyo-rust/actions/workflows/ci.yml)
[latest release](https://github.com/pnsk-lab/xyo-rust/releases/latest)

Scratch の `.sb3` プロジェクトを読み込み、解析し、LLVM IR を生成して JIT で動かす実験プロジェクト `xyo-rust` の手引きです。

## このドキュメントについて

このドキュメントは `xyo-rust` の利用者・開発者向けの技術解説書です。Scratch プロジェクトのコンパイル基盤として `xyo-rust` がどのように動作するかを、セットアップから内部アーキテクチャまで段階的に説明します。

## 目次

- [はじめに](./README.md) ← 現在のページ
- [セットアップ](./getting-started.md)
- [CLI](./cli.md)
- [対応ブロック一覧](./blocks.md)
- [アーキテクチャ](./architecture.md)

## Scratch と `.sb3` について

[Scratch](https://scratch.mit.edu/) は MIT メディアラボが開発したビジュアルプログラミング環境で、ブロックをドラッグ＆ドロップして組み合わせることでプログラムを作れます。

Scratch プロジェクトは `.sb3` という拡張子で保存されます。`.sb3` の正体は ZIP アーカイブで、内部に `project.json` というファイルがあり、すべてのブロック・スプライト・変数・コスチューム・サウンドのメタデータが JSON 形式で記録されています。

### `project.json` の構造

`project.json` の最上位には `targets` という配列があり、各要素がスプライトまたはステージを表します。各ターゲットは次の情報を持ちます。

| フィールド | 説明 |
| ---------- | ---- |
| `blocks` | ブロックの辞書。ブロック ID をキーにしてブロック情報が格納される |
| `variables` | 変数の辞書。変数 ID をキーに名前と初期値が入る |
| `lists` | リストの辞書。リスト ID をキーに名前と初期値が入る |
| `costumes` | コスチューム情報の配列 |
| `sounds` | サウンド情報の配列 |
| `isStage` | このターゲットがステージかどうか |

各ブロックは次のフィールドを持ちます。

| フィールド | 説明 |
| ---------- | ---- |
| `opcode` | ブロックの種類を表す識別子 (`motion_movesteps` など) |
| `inputs` | ブロックへの入力（数値・文字列・他のブロックへの参照） |
| `fields` | ドロップダウンメニューの選択値 |
| `next` | 次に実行するブロックの ID（連鎖構造） |
| `parent` | 親ブロックの ID |
| `topLevel` | スクリプトの先頭ブロックかどうか |

## このプロジェクトが目指すこと

`xyo-rust` は以下を目標としています。

- Scratch プロジェクトを **静的解析** できる内部表現へ変換する
- hat block からスレッドを抽出し、実行単位を定義する
- LLVM IR を生成し、現在は JIT 実行で挙動確認を行う
- SB3 解析系の **検証用ツールチェーン** を整える

完全な Scratch 互換ランタイムを作るのではなく、「Scratch プロジェクトをどこまで静的に処理できるか」を探ることが現在の中心です。

## 現在の実装範囲

| 段階 | 内容 | 状態 |
| ---- | ---- | ---- |
| 1 | `.sb3` を ZIP として開き `project.json` を取り出す | ✅ 完成 |
| 2 | `project.json` を Scratch プロジェクト構造へ変換する | ✅ 完成 |
| 3 | hat block からスレッドを解析する | ✅ 完成 |
| 4 | `Stmt` / `Expr` に変換する（パーサー） | ✅ ほぼ完成（86 opcode） |
| 5 | 一部の式 / 文を LLVM IR へ変換し、JIT で実行する | 🚧 一部実装（動き系 + 演算子） |
| 6 | 生成した IR から実行可能ファイルを作る | ❌ 未実装 |

```warn
完全な Scratch 実行互換はまだ未実装です。特に `run` サブコマンドは実験段階で、入力によっては未実装分岐に到達します。成功時は各スレッドの状態が標準出力に表示されます。
```

## 処理の流れ（概要）

`xyo-rust` は Scratch プロジェクトをネイティブコードへ変換するため、以下の 4 段階のパイプラインを通します。

```
.sb3
 │
 ▼
SB3 ロード（src/sb3.rs）
 │  ZIP を展開して project.json を読み込む
 │
 ▼
デシリアライズ（src/types/）
 │  JSON → Rust の ScratchProject 構造体
 │
 ▼
パース（src/parser/）
 │  hat block → Thread (スレッド)
 │  各ブロック → Stmt / Expr (AST)
 │
 ▼
IR 生成（src/compiler/）
   Thread → LLVM 関数
   Stmt/Expr → LLVM IR 命令列
   最適化パス（O3）を適用
   JIT で各スレッドを実行
   → 標準出力へ実行結果を表示
```

詳細は [アーキテクチャ](./architecture.md) を参照してください。

## 読み進め方

目的に合わせて読み始めるページを選んでください。

| 目的 | 参照ページ |
| ---- | ---------- |
| まず動かしてみたい | [セットアップ](./getting-started.md) |
| コマンドの使い方を知りたい | [CLI](./cli.md) |
| 対応済み opcode を確認したい | [対応ブロック一覧](./blocks.md) |
| 内部構造を把握したい | [アーキテクチャ](./architecture.md) |
| コントリビュートしたい | [アーキテクチャ](./architecture.md) → [対応ブロック一覧](./blocks.md) |

## 現時点での注意点

- Scratch VM との完全互換は未実装です
- LLVM 21.1.x 系を前提にしています（他のバージョンではビルドが失敗します）
- 配布用の `.sb3` サンプルは同梱されていません。Scratch エディタからエクスポートしてください
- `run` サブコマンドは実験的で、動き系 + 演算子のみ対応しています

## 関連リンク

- [リポジトリの README](../README.md)
- [GitHub](https://github.com/pnsk-lab/xyo-rust)
- [Scratch 公式サイト](https://scratch.mit.edu/)
- [LLVM 公式サイト](https://llvm.org/)
- [inkwell (LLVM Rust バインディング)](https://github.com/TheDan64/inkwell)

続きは [セットアップ](./getting-started.md) を参照してください。
