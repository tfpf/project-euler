FROM --platform=linux/i386 rust:1.75-alpine3.19
WORKDIR /project-euler
COPY . .
