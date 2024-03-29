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

# build CSS
FROM node:alpine as sass
COPY ./styles ./styles
RUN npm install -g sass
RUN sass ./styles/index.scss ./styles.css

# our final base
FROM rust:1.39

# copy assets for runtime
COPY csv ./csv/
COPY static ./static/

# copy the build artifact from the build stage
COPY --from=build /tcrec/target/release/tcrec .
COPY --from=sass styles.css ./static/

CMD ["./tcrec"]