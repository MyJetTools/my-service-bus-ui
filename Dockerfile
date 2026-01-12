FROM myjettools/dioxus-docker:0.7.2

ENV PORT=9001
ENV IP=0.0.0.0

COPY ./target/dx/my-sb-ui/release/web /target/dx/my-sb-ui/release/web

RUN chmod +x /target/dx/my-sb-ui/release/web/my-sb-ui
WORKDIR /target/dx/my-sb-ui/release/web/
ENTRYPOINT ["./my-sb-ui"]