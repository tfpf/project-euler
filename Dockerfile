FROM --platform=linux/i386 alpine
RUN apk add cargo
COPY . /
WORKDIR /
ENTRYPOINT cargo run --release
