# This is used to run tests in a 32-bit Docker container.
FROM --platform=linux/i386 alpine
RUN apk add cargo
COPY . /
WORKDIR /
RUN cargo run --release
