FROM rust:slim
COPY ./target/release/rust-service-template ./target/release/rust-service-template 
ENTRYPOINT ["./target/release/rust-service-template"]