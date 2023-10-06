FROM rust:1.70

COPY ./editor-server ./editor-server
COPY ./editor-server/.env.production ./editor-server/.env

WORKDIR /editor-server

EXPOSE 4000
CMD ["bash", "-c", "cargo prisma migrate deploy && cargo run --release"]
