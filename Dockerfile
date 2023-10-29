FROM ubuntu:22.04

COPY ./target/release/my-sb-ui ./target/release/my-sb-ui
COPY ./dist ./dist
ENTRYPOINT ["./target/release/my-sb-ui"]