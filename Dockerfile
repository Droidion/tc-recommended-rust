# select image
FROM rust:1.39 as build

# create a new empty shell project
RUN USER=root cargo new --bin tcrec
WORKDIR /tcrec

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree and templates
COPY ./src ./src
COPY ./templates ./templates

# build for release
RUN rm ./target/release/deps/tcrec*
RUN cargo build --release

# our final base
FROM rust:1.39

# copy the build artifact from the build stage
COPY --from=build /tcrec/target/release/tcrec .

# copy assets for runtime
COPY csv ./csv/
COPY static ./static/


CMD ["./tcrec"]