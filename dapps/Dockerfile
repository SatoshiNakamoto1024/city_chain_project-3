# ベースイメージを指定
FROM python:3.9-slim

# 作業ディレクトリを設定
WORKDIR /usr/src/app

# 必要なパッケージをコピーしてインストール
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

# 必要なパッケージをインストール
RUN apt-get update && apt-get install -y \
    gcc \
    libssl-dev \
    git \
    cargo \
    && rm -rf /var/lib/apt/lists/*

# city_chain_project-2リポジトリからntruフォルダーをクローン
RUN git clone https://github.com/SatoshiNakamoto1024/city_chain_project-3.git /usr/src/city_chain_project-3 && \
    cp -r /usr/src/city_chain_project-3/ntru /usr/src/app/ntru

# NTRUクレートの依存関係をコピーしてビルド
RUN cd /usr/src/app/ntru && cargo build --release

# アプリケーションのソースコードをコピー
COPY . .

# Flaskアプリケーションの起動
CMD ["python", "app.py"]
