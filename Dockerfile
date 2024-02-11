FROM rust:1.75-alpine
RUN apk add cargo
WORKDIR /project-euler
COPY . .
