FROM rust:1.79
WORKDIR /usr/src/enclave-notary-server
COPY . .

RUN cargo install --path .

CMD ["enclave-notary-server"]
