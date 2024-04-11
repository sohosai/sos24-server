# sos24-server
[![CD(prodution)](https://github.com/sohosai/sos24-server/actions/workflows/cd.yaml/badge.svg?event=pull_request)](https://github.com/sohosai/sos24-server/actions/workflows/cd.yaml)
[![CD(staging)](https://github.com/sohosai/sos24-server/actions/workflows/cd-staging.yaml/badge.svg?event=pull_request)](https://github.com/sohosai/sos24-server/actions/workflows/cd-staging.yaml)


> [!NOTE]
> クエリを変更した場合はCIを通すために `cargo sqlx prepare --workspace` を実行してください。

## sos24-presentation

sos24-presentationクレートは、アプリケーションのHTTPサーバーとしての役割を果たしています。このクレートは、HTTPリクエストを適切なユースケースにルーティングし、その結果をHTTPレスポンスとしてクライアントに返します。

具体的には、create_app関数でHTTPルーターを作成し、それぞれのエンドポイント（例えば、ニュースやユーザーに関するエンドポイント）を適切なハンドラー関数にルーティングします。これらのハンドラー関数は、Modules構造体を通じてユースケースにアクセスします。

Modules構造体は、アプリケーションの設定と、ニュースとユーザーに関するユースケースを保持します。これらのユースケースは、sos24-use-caseクレートで定義されています。

また、ToStatusCodeトレイトは、エラーをHTTPステータスコードに変換するために使用されます。

## sos24-use-case

sos24-use-caseクレートは、アプリケーションのビジネスロジックを実装しています。このクレートは、アプリケーションのユースケース（操作）を表現するためのインターフェースを提供します。

具体的には、UserUseCaseとNewsUseCaseの2つの主要なユースケースがあります。

UserUseCaseは、ユーザーに関連する操作を提供します。これには、ユーザーの作成（createメソッド）、特定のIDを持つユーザーの検索（find_by_idメソッド）などが含まれます。

一方、NewsUseCaseは、ニュースに関連する操作を提供します。これには、ニュースの作成（createメソッド）、ニュースのリスト取得（listメソッド）などが含まれます。

これらのユースケースは、Repositoriesトレイトを通じてデータストレージにアクセスします。このトレイトは、sos24-domainクレートで定義されたエンティティを操作するためのメソッドを提供します。

また、このクレートは、UserDtoやCreateUserDtoなどのデータ転送オブジェクト（DTO）を定義しています。これらのDTOは、ユースケースとプレゼンテーション層との間でデータをやり取りするために使用されます。

## sos24-domain

sos24-domainクレートは、アプリケーションのドメインモデルを定義しています。このクレートは、アプリケーションのビジネスロジックの核心部分を表現するエンティティと、それらのエンティティを永続化するためのリポジトリインターフェースを提供します。

具体的には、entityモジュールとrepositoryモジュールが主要な部分です。

entityやNewsなどのエンティティが定義されています。これらのエンティティは、アプリケーションのビジネスルールをカプセル化し、アプリケーションの状態を表現します。

一方、repositoryモジュールでは、これらのエンティティを永続化するためのインターフェースが定義されています。これにより、ビジネスロジックは永続化の詳細から分離され、異なる永続化メカニズムを容易に切り替えることができます。

また、このクレートはtestモジュールも提供しています。これは、ドメインモデルの単体テストをサポートするためのユーティリティを提供します。

このように、sos24-domainクレートは、アプリケーションのビジネスロジックを表現し、そのロジックをテストし、永続化するための基盤を提供します。

## sos24-infrastructure

sos24-infrastructureクレートは、アプリケーションのインフラストラクチャ層を定義しています。このクレートは、アプリケーションのビジネスロジックをサポートするための具体的な永続化メカニズムと外部サービスの実装を提供します。

具体的には、firebaseモジュールとpostgresqlモジュールが主要な部分です。

firebaseモジュールでは、Firebaseを使用したユーザー認証の実装が提供されています。FirebaseUserRepositoryImplクラスは、Firebaseを使用してユーザー情報を取得するための具体的なメカニズムを提供します。

一方、postgresqlモジュールでは、PostgreSQLを使用したデータの永続化の実装が提供されています。PgNewsRepositoryやPgUserRepositoryクラスは、それぞれニュースとユーザー情報をPostgreSQLデータベースに永続化するための具体的なメカニズムを提供します。

また、DefaultRepositoriesクラスは、これらの具体的なリポジトリの実装をまとめ、sos24-domainクレートで定義されたリポジトリインターフェースを満たす形で提供します。

このように、sos24-infrastructureクレートは、アプリケーションのビジネスロジックをサポートするための具体的なインフラストラクチャを提供します。
