# セットアップ

## 前提条件

- Rust stable
- LLVM 21.1 系
- `llvm-config` が PATH 上にある、または適切な環境変数が設定されていること

Linux / macOS / Windows では、CI と同様に LLVM 21.1 系を使う想定です。

## ビルド

```bash
cargo build --release
```

## テスト

```bash
cargo test
```

## 主な入力ファイル

`examples/` には動作確認用の `.sb3` サンプルが含まれています。

例:

```bash
cargo run -- stats examples/simple.sb3
```

## GitHub Pages 公開

このドキュメントは `mdBook` で生成され、GitHub Actions で公開されます。

1. リポジトリの **Settings > Pages** を開く
2. **Source** または **Build and deployment** で **GitHub Actions** を選ぶ
3. `docs/` や `README.md` を変更して push する、またはタグ付きの GitHub Release を publish する
4. Actions の `Docs` ワークフロー完了後、`https://<owner>.github.io/<repo>/` で公開される

独自ドメインを使う場合は、GitHub Pages 側で追加設定してください。
