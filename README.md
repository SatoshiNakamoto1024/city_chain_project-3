# City Chain Project

市町村のブロックチェーンのプログラム

## 概要
City Chain Projectは、市町村、地域、大陸、そしてグローバルレベルでのトランザクションを処理するためのブロックチェーンシステムです。このプロジェクトは、市町村ごとのチェーン、地域チェーン、大陸チェーン、そしてグローバルメインチェーンで構成されています。

## プロジェクト構成
- **Global Main Chain**: 大陸チェーンからのデータを集約するメインチェーン。
- **Continental Chain**: 市町村チェーンからのデータを集約し、メインチェーンに転送する中間チェーン。
- **Municipal Chain**: 市町村内でのトランザクションを処理するローカルチェーン。
- **DApps**: 分散型アプリケーションで、ユーザーがトランザクションを作成し、システムとインタラクションするためのインターフェースを提供します。
- **Mobile App**: DAppsのモバイルバージョンで、移動中のアクセスを提供します。
- **Lattice Signer Service**: トランザクションの署名と検証を行うサービス。

## セキュリティ統合
- **NTRU Encryption and Signatures**: トランザクションのセキュリティと署名の検証に使用。ラティスベースの暗号技術を用いることで、量子計算に対する耐性を持ち、システム全体のセキュリティを強化します。
- **Verifiable Credentials**: ユーザーのクレデンシャルの認証と整合性を確保するために使用。これにより、ユーザーの身元と行動の信頼性を保証し、システム全体の信頼性を向上させます。

## 環境セットアップ
1. リポジトリをクローンします：
    ```bash
    git clone https://github.com/your-repo/city_chain_project.git
    cd city_chain_project-3
    ```

2. Dockerを使用してプロジェクトをビルドし、起動します：
    ```bash
    docker-compose up --build
    ```

3. 各サービスが以下のポートで実行されます：
    - Global Main Chain: `http://localhost:8000`
    - Continental Chain: `http://localhost:8001`
    - Municipal Chain: `http://localhost:8002`
    - DApps: `http://localhost:5000`
    - Lattice Signer Service: `http://localhost:5001`
    - Mobile App: `http://localhost:3000`

## 使用方法
各サービスにはそれぞれのエンドポイントがあります。以下はMunicipal Chainの例です：

- **POST /transaction**
  - 新しいトランザクションを作成します。
  - リクエストボディ:
    ```json
    {
      "sender": "string",
      "receiver": "string",
      "amount": "float"
    }
    ```
  - レスポンス:
    ```json
    {
      "transaction_id": "string",
      "timestamp": "string"
    }
    ```

- **POST /add_block**
  - 新しいブロックを追加します。
  - リクエストボディ:
    ```json
    {
      "index": "integer",
      "timestamp": "string",
      "data": "string",
      "prev_hash": "string",
      "hash": "string"
    }
    ```
  - レスポンス:
    ```json
    {
      "status": "string"
    }
    ```

## 貢献方法
1. このリポジトリをフォークします。
2. 新しいブランチを作成します (`git checkout -b feature-branch`)。
3. 変更をコミットします (`git commit -am 'Add new feature'`)。
4. ブランチをプッシュします (`git push origin feature-branch`)。
5. プルリクエストを開きます。

## ライセンス
このプロジェクトはMITライセンスの下でライセンスされています。詳細については、`LICENSE` ファイルを参照してください。

## その他
このプロジェクトに関する質問や提案がある場合は、[Issues](https://github.com/SatoshiNakamoto1024/city_chain_project/issues) セクションを通じてお問い合わせください。
