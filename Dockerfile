# Start with a rust alpine image
FROM rust:1-alpine3.16
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
#ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
RUN apk update
RUN apk add pkgconfig
RUN apk add protoc
RUN apk add --no-cache musl-dev
RUN apk add --update openssl
RUN apk add --update openssl-dev
# set the workdir and copy the source into it
RUN update-ca-certificates

WORKDIR /app
COPY ./ /app
# do a release build
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN strip target/x86_64-unknown-linux-musl/release/rust-service-template

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.16
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc
# copy the binary into the final image
COPY --from=0 app/target/x86_64-unknown-linux-musl/release/rust-service-template ./
RUN chmod 777 rust-service-template
# set the binary as entrypoint
ENTRYPOINT ["/rust-service-template"]