FROM rust:1.70

RUN cargo new --bin editor-server
WORKDIR /editor-server

COPY ./editor-server/Cargo.toml .
COPY ./editor-server/Cargo.lock .

COPY ./editor-server/.cargo ./.cargo
COPY ./editor-server/prisma ./prisma

RUN cargo build -p prisma
RUN cargo prisma --version

RUN cargo fetch
RUN rm -r ./src

COPY ./editor-server/.env.production ./.env
COPY ./editor-server/src ./src

EXPOSE 4000
CMD ["bash", "-c", "cargo prisma migrate deploy && cargo run --release"]
# CMD ["bash", "-c", "ls ./src -la"]
