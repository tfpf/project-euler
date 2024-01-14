FROM --platform=linux/i386 alpine-3.19
RUN apk add cargo
COPY . /
WORKDIR /
ENTRYPOINT cargo run --release
