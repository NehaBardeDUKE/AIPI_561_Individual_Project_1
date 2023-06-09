FROM rust:latest as builder
ENV APP onnx_pytorch_mod
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
 
FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/* 
RUN apt-get update && apt-get install -y libssl-dev
RUN apt-get update && \
    apt-get install -y unzip

COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP


ENTRYPOINT [ "/usr/local/bin/onnx_pytorch_mod" ]
