# ベースイメージとして公式のRustランタイムを使用
FROM rust:latest AS builder

# 作業ディレクトリを設定
WORKDIR /usr/src/app

# 必要なパッケージをインストール
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev

# 依存関係をコンテナにコピー
COPY Cargo.toml Cargo.lock ./

# 依存関係をビルド
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 残りのソースコードをコンテナにコピー
COPY . .

# アプリケーションをビルド
RUN cargo build --release

# 実行可能ファイルをエントリーポイントとして設定
FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/global_main_chain /usr/local/bin/global_main_chain
CMD ["/usr/local/bin/global_main_chain"]


