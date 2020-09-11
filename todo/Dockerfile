FROM rust:1.43 AS builder

WORKDIR /todo

# Cargo.tomlのみをイメージに追加
COPY Cargo.toml Cargo.toml
# 何もしないソースコードを追加
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
# 何もしないソースコードとdependencyをビルド
RUN cargo build --release

# アプリケーションのソースコードを追加
COPY ./src ./src
COPY ./templates ./templates
# 何もしないソースコードからの生成物を削除（それ以外は残してビルドを高速化）
RUN rm -f target/release/deps/todo*
# アプリケーションのソースコードの部分のみをビルド
RUN cargo build --release

# リリース用のイメージを作成
FROM debian:10.4
COPY --from=builder /todo/target/release/todo /usr/local/bin/todo
CMD ["todo"]
