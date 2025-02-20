FROM ubuntu:22.04

ENV PORT=9001
ENV IP=0.0.0.0

EXPOSE 9001

COPY ./target/dx/my-sb-ui/release/web /target/dx/my-sb-ui/release/web
RUN chmod +x /target/dx/my-sb-ui/release/web/server
WORKDIR /target/dx/my-sb-ui/release/web/
ENTRYPOINT ["./server" ]