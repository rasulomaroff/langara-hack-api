FROM rust:1.81.0 as builder

RUN USER=root cargo new --bin htmlgen
WORKDIR /htmlgen

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/htmlgen*
RUN cargo install --path .

EXPOSE 8000

CMD ["htmlgen"]
