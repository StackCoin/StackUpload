FROM ekidd/rust-musl-builder AS builder

ADD . ./

RUN sudo chown -R rust:rust /home/rust

RUN cargo build --release

FROM alpine:latest
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/stackupload \
    /usr/local/bin/

WORKDIR /app
RUN mkdir ./upload

CMD /usr/local/bin/stackupload

