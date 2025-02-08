# sos24-server

[![CD(prodution)](https://github.com/sohosai/sos24-server/actions/workflows/cd.yaml/badge.svg)](https://github.com/sohosai/sos24-server/actions/workflows/cd.yaml)
[![CD(beta)](https://github.com/sohosai/sos24-server/actions/workflows/cd-beta.yaml/badge.svg)](https://github.com/sohosai/sos24-server/actions/workflows/cd-beta.yaml)
[![CD(staging)](https://github.com/sohosai/sos24-server/actions/workflows/cd-staging.yaml/badge.svg)](https://github.com/sohosai/sos24-server/actions/workflows/cd-staging.yaml)

雙峰祭オンラインシステムのサーバーです。

> [!NOTE]
> クエリを変更した場合はCIを通すために `cargo sqlx prepare --workspace` を実行してください。

## API リファレンス

- https://sohosai.github.io/sos24-server/

## 開発方法

### 環境構築

VSCodeのDevcontainerを使用することを推奨します。この場合、以下のツールがインストールされている必要があります。

- VSCode
- Docker
- Docker Compose

### 環境変数

`.env.sample`を参考に`.env`ファイルを作成し、環境変数を設定してください。
変更する可能性の高い環境変数を以下に示します。その他の環境変数については現在のクレデンシャルを参照してください。

| 環境変数名 | 説明 | 例 |
| --- | --- | --- |
| `PORT` | ポート番号 | `8080` |
| `FIREBASE_PROJECT_ID` | FirebaseのプロジェクトID | |
| `FIREBASE_PRIVATE_KEY` | Firebaseの秘密鍵 | |
| `REQUIRE_EMAIL_VERIFICATION` | メールアドレスの確認を必須にするかどうか | `true`,`false` |
| `PROJECT_APPLICATION_START_AT` | 企画応募開始日時(RFC3339) | `2024-03-15T00:00:00+09:00` |
| `PROJECT_APPLICATION_END_AT` | 企画応募終了日時(RFC3339) | `2024-04-15T22:00:00+09:00` |
| `SEND_GRID_API_KEY` | SendGridのAPIキー | |
| `EMAIL_SENDER_ADDRESS` | メール送信時にSenderに設定するメールアドレス | |
| `EMAIL_REPLY_TO_ADDRESS` | メール送信時にReply-Toに設定するメールアドレス | |
| `APP_URL` | sos24-clientがデプロイされたURL | `https://sos24.sohosai.com` |

### マイグレーション

`cargo install sqlx-cli`で`sqlx-cli`をインストールします。
その後`cargo sqlx database create`でデータベースを作成し、`cargo sqlx migrate run`でマイグレーションを実行します。

### ビルド

`cargo run --bin sos24-presentation`でサーバーを起動します。

### テスト

`cargo test`もしくは`cargo nextest run`でテストを実行します。
