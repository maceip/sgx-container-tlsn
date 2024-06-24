FROM rust:1.79
WORKDIR /usr/src/enclave-notary-server
COPY . .

RUN cargo install --path .
#DEBUG
COPY /data/sealed /data/sealed 
CMD ["enclave-notary-server"]
