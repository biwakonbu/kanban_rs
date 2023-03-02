# README

## 参考ページ

- [YewとaxumとShuttleで RealWorld example を書いてみた](https://zenn.dev/tanakh/articles/realworld-zum-yew-shuttle)  
  axum の導入の参考に

## 参考リポジトリ

- [tanakh/axum-yew-shuttle-realworld-example-app](https://github.com/tanakh/axum-yew-shuttle-realworld-example-app)  
  エラーハンドリングの導入の参考に利用
## Install

特にやる事はない。コンパイル確認するなら check を使う。

``` shellsession
$ cargo check
```

## Tools

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
