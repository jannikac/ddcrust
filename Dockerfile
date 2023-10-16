FROM rust:1.73 as build

# create a new empty shell project
RUN USER=root cargo new --bin ddcrust
WORKDIR /ddcrust

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/ddcrust*
RUN cargo build --release

# our final base
FROM debian:bookworm-slim
WORKDIR /app/ddcrust
RUN apt-get update && apt-get install libssl-dev ca-certificates -y && apt-get clean
# copy the build artifact from the build stage
COPY --from=build /ddcrust/target/release/ddcrust .

# add non-root user
RUN addgroup --gid 1001 --system ddcrust && \
    adduser --gid 1001 --uid 1001 --system --shell /sbin/nologin ddcrust && \
    chown -R ddcrust:ddcrust /app/ddcrust

USER ddcrust

# set the startup command to run your binary
CMD ["./ddcrust"]
