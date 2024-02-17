FROM --platform=linux/i386 alpine:3.19
RUN apk add cargo rustup
RUN rustup update
WORKDIR /project-euler
COPY . .
