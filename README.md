# CORS デモンストレーションプロジェクト

このプロジェクトは、Go、Rust、Python（FastAPI）を使用したマルチサーバーセットアップで、Cross-Origin Resource Sharing（CORS）の動作をデモンストレーションします。異なるオリジン間でのWebリクエストに対するCORSポリシーの影響を実践的に示します。

## プロジェクト構造

- `go_server/`: メインのHTMLページを提供し、プロキシとして機能するGoサーバー
- `rust_server/`: CORSが有効になっていないエンドポイントを示すRustサーバー
- `python_server/`: CORSが有効になっているPython（FastAPI）サーバー
- `docker-compose.yml`: すべてのサーバーを実行するためのDocker Compose設定
- `Makefile`: Docker Composeコマンドを簡略化

## 前提条件

- Docker
- Docker Compose

## セットアップと実行

1. このリポジトリをクローンします：
   ```
   git clone https://github.com/your-username/cors-demonstration.git
   cd cors-demonstration
   ```

2. Docker Composeを使用してサーバーを起動します：
   ```
   make up
   ```
   または
   ```
   docker-compose up --build
   ```

3. Webブラウザを開き、`http://localhost:8080` にアクセスします

## 使用方法

メインページ（`http://localhost:8080`）には、異なるCORSシナリオを示す4つのボタンがあります：

1. **同一オリジン (Go)**: Goサーバーへの同一オリジンリクエスト
2. **別オリジン but プロキシ (Rust via Go)**: Rustサーバーへのクロスオリジンリクエスト（Goサーバー経由でプロキシ）
3. **別オリジン No CORS (Rust)**: Rustサーバーへの直接のクロスオリジンリクエスト（CORS無効）
4. **別オリジン Yes CORS (Python/FastAPI)**: Pythonサーバーへのクロスオリジンリクエスト（CORS有効）

各ボタンをクリックして、CORSがリクエストにどのように影響するかを確認してください。

## サーバーの役割

- **Goサーバー（ポート8080）**:
  - メインのHTMLページを提供
  - 同一オリジンリクエストを処理
  - Rustサーバーへのプロキシとして機能

- **Rustサーバー（ポート8081）**:
  - CORSが有効になっていないサーバーをデモンストレーション

- **Pythonサーバー（ポート8082）**:
  - FastAPIを使用してCORSが有効になっているサーバーをデモンストレーション

## 注意事項

- "別オリジン No CORS (Rust)" リクエストは、CORS制限により失敗することが予想されます
- ブラウザのコンソールで詳細なCORS関連のエラーメッセージを確認してください

## 貢献

デモンストレーションの改善や機能拡張への貢献を歓迎します。変更を提案する場合は、プルリクエストを提出するか、イシューを開いて議論してください。

## ライセンス

[MITライセンス](LICENSE)