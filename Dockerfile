FROM rustlang/rust:nightly

WORKDIR /usr/src/superworldcon

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/superworldcon"]
