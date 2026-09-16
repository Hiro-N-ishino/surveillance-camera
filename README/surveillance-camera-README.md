# Raspberry Pi 監視カメラシステム

Raspberry Pi上で動作する監視カメラシステムです。

Rustでカメラ撮影、温湿度取得、API通信、ローカルファイル管理を実装し、
Cloudflare Workers APIを経由してCloudflare R2へ画像と撮影時のデータを保存します。

Androidアプリから最新画像と温湿度履歴を参照できます。

## システム構成

![データフロー](README/data-flow.jpg)

本システムは、Raspberry Pi上のRustアプリケーション、Cloudflare Workers API、
Cloudflare R2、Androidアプリで構成されています。

### データの流れ

- Raspberry PiからCloudflare Workers APIへ画像と撮影時のメタデータをHTTPSで送信
- Cloudflare Workers APIが画像とメタデータをCloudflare R2へ保存
- AndroidアプリからCloudflare Workers APIへ画像・温湿度データを取得
- Raspberry PiとAndroidアプリは直接通信せず、Cloudflare Workers APIを経由して通信

## 処理フロー

![処理フロー](README/processing-flow.jpg)

## 主な機能

- Raspberry Pi Camera Module 3による画像撮影
- DHT11による温度・湿度の取得
- Raspberry Pi側での撮影日時の取得
- Cloudflare Workers APIへのHTTPSアップロード
- Cloudflare R2への画像・撮影日時・温湿度データの保存
- Basic認証によるAPI認証
- 過去48時間分の温湿度履歴取得
- systemdによる常駐実行
- 連続エラー発生時のプロセス終了とsystemdによる再起動

## 使用技術

| 分類 | 技術 |
|---|---|
| OS | Raspberry Pi OS / Debian 13 |
| 言語 | Rust |
| カメラ | Raspberry Pi Camera Module 3 |
| センサー | DHT11 |
| API | Cloudflare Workers |
| ストレージ | Cloudflare R2 |
| 認証 | HTTP Basic認証 |
| プロセス管理 | systemd |

## Rustアプリケーション

### 処理内容

1. カメラで画像を撮影
2. DHT11から温度・湿度を取得
3. 撮影日時、ファイル名、温度、湿度から`Observation`を生成
4. 画像とメタデータをCloudflare Workers APIへアップロード
5. アップロード試行後、Raspberry Pi上のJPEGファイルを削除
6. 設定した間隔だけ待機
7. 次の撮影サイクルへ移行

### データモデル

1回の撮影結果を`Observation`として管理しています。

```text
file_name
captured_at
temperature
humidity
```

`captured_at`はRaspberry Pi側で撮影時に生成し、
バックエンドへ明示的に送信しています。

ファイル名から撮影日時を推測するのではなく、
実際の撮影時刻をデータとして扱う構成にしています。

## API通信

Raspberry PiからCloudflare Workers APIへ、以下の情報を送信します。

```text
POST /?filename=<file_name>

Authorization: Basic ...
X-Captured-At: ...
X-Temperature: ...
X-Humidity: ...
```

Cloudflare Workers APIでは、画像とともに以下のメタデータをCloudflare R2へ保存します。

```text
capturedAt
temperature
humidity
```

## エラー処理

撮影からアップロードまでを1つの処理サイクルとして扱い、
サイクル単位でエラーを処理しています。

エラーが発生した場合は、

- エラー内容をログへ出力
- 連続エラー回数を加算
- 設定した閾値に達した場合はアプリケーションを終了
- systemdによってプロセスを再起動

という流れで復旧します。

また、アップロードに失敗した場合でも、
そのサイクルで作成したJPEGファイルだけを削除します。

## モジュール構成

```text
src/
├── main.rs
├── camera.rs
├── capture_cycle.rs
├── config.rs
├── fs.rs
├── observation.rs
├── sensor.rs
└── upload.rs
```

| ファイル | 主な役割 |
|---|---|
| `main.rs` | アプリケーション全体の制御、エラー回数管理、撮影サイクルの実行 |
| `camera.rs` | カメラ操作 |
| `capture_cycle.rs` | 1回の撮影処理全体の制御 |
| `config.rs` | 設定値の読み込み |
| `fs.rs` | ファイル名生成などのファイル処理 |
| `observation.rs` | 撮影結果を表すデータ構造 |
| `sensor.rs` | DHT11からの温湿度取得 |
| `upload.rs` | Cloudflare Workers APIへのアップロード |

## 設計上のポイント

### 役割分担

カメラ撮影、センサー取得、ファイル処理、データモデル、API通信、アプリケーション制御をRustのモジュール単位で分離しています。

### 撮影日時の明示的な管理

撮影日時はRaspberry Pi側で取得し、`X-Captured-At`ヘッダーとしてバックエンドへ送信しています。

Cloudflare R2ではこの値を撮影日時の情報として保存し、
過去データの取得や並び順の判定にも利用しています。

既存のデータについては撮影日時のメタデータが存在しない場合に、
ファイル名から日時を取得するフォールバック処理も実装しています。

### ローカルJPEGの管理

撮影したJPEGはRaspberry Pi上で一時ファイルとして扱っています。

アップロードの成否にかかわらず、現在の処理サイクルで作成したJPEGだけを削除することで、
アップロード失敗時にも不要なファイルが蓄積しないようにしています。

## 関連プロジェクト

Android側の参照アプリケーション：

`surveillance-camera-app`
