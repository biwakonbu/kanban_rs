# README

## 参考ページ

- [YewとaxumとShuttleで RealWorld example を書いてみた](https://zenn.dev/tanakh/articles/realworld-zum-yew-shuttle)  
  axum の導入の参考に

## 参考リポジトリ

- [tanakh/axum-yew-shuttle-realworld-example-app](https://github.com/tanakh/axum-yew-shuttle-realworld-example-app)  
  エラーハンドリングの導入の参考に利用

## 実装時に気になった事

### axum の Router を機能毎に mod 分割した場合、with_state で Pool<MySql> みたいなコネクションをどうするのか問題

引数で渡して呼び出し先で `with_state(pool)` みたいな事をやると main.rs の main 内で起動するメインの Router に合流する際に `pool` が使えなくて困る。  
それを解決できる手段は clone でそもそも実体への参照を分離する事だけど、その場合心配になるのは connection が増加してしまうのでは？という問題。

Router を分割した数だけ接続が勝手に増えてしまうのは嫌なので、どうしたものかと思ったら、同様の事を issue で質問している人がいて、`Arc<SharedPool<DB>>` 型だから DB 接続は増えないという事が説明されていた。

- [tweet - @biwakobu: axum で Router を機能毎に mod 分割したくて分けたら...](https://twitter.com/biwakonbu/status/1631592973472534529?s=20)

## Install

特にやる事はない。コンパイル確認するなら check を使う。

``` shellsession
$ cargo check
```

## Tools

### sqlx-cli
DB アクセスには sqlx を使うため、マイグレーション用に sqlx-cli を導入する。
`.env` ファイルに `DATAABASE_URL` を設定する事で使用可能。

``` shellsession
# コマンドインストール
$ cargo install sqlx-cli
```

sqlx-cli 使い方
``` shellsession
# DB 作成
$ sqlx-cli database create
# DB 削除
$ sqlx-cli database drop
# migrate ファイル作成
$ sqlx-cli migrate add <migrate-name>
# migrate ファイル作成 (revert ファイル付き)
$ sqlx-cli migrate add -r <migrate-name>
# migration 実行
$ sqlx-cli migrate run
# ロールバック (1つずつ)
$ sqlx-cli migrate revert
```

### xh

CUI でのリクエストチェックに。curl 以外のリッチなツールとして。

インストール

``` shellsession
$ cargo install xh
```

利用例

``` shellsession
# GET
$ xh get http://localhost:3000/users
# POST
$ xh post http://localhost:3000/users username=biwakonbu
```

## Setup

``` shellsession
# MySQL 起動
$ docker compose up
# 環境変数ファイル作成
$ cp .env.example .env
# DB 作成
$ sqlx-cli migrate create
# migration 実行
$ sqlx-cli migrate run
```
