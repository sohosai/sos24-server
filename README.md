# sos24-server

[![CD(prodution)](https://github.com/sohosai/sos24-server/actions/workflows/cd.yaml/badge.svg)](https://github.com/sohosai/sos24-server/actions/workflows/cd.yaml)
[![CD(beta)](https://github.com/sohosai/sos24-server/actions/workflows/cd-beta.yaml/badge.svg)](https://github.com/sohosai/sos24-server/actions/workflows/cd-beta.yaml)
[![CD(staging)](https://github.com/sohosai/sos24-server/actions/workflows/cd-staging.yaml/badge.svg)](https://github.com/sohosai/sos24-server/actions/workflows/cd-staging.yaml)

雙峰祭オンラインシステムのサーバーです。

> [!NOTE]
> クエリを変更した場合はCIを通すために `cargo sqlx prepare --workspace` を実行してください。

## API リファレンス
- https://sos24.readme.io/reference/getprojectapplicationperiod (要ログイン)

## 環境構築

### 環境変数

`.env.sample`を参考に`.env`ファイルを作成し、環境変数を設定してください。

### データベースのセットアップ

`cargo install sqlx-cli`で`sqlx-cli`をインストールします。その後`cargo sqlx database create`でデータベースを作成し、`cargo sqlx migrate run`でマイグレーションを実行します。

### ビルド

`cargo run --bin sos24-presentation`でサーバーを起動します。

### テスト

`cargo test`もしくは`cargo nextest run`でテストを実行します。
