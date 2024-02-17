FROM --platform=linux/i386 alpine:3.19
RUN apk add cargo
WORKDIR /project-euler
COPY . .
