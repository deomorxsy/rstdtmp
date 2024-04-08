FROM rust:1.77.1-alpine3.19 as builder

#
# build stage
#
WORKDIR /usr/src/mcl/
COPY . .
COPY .env.docker .env

RUN cargo install --path .

#
# deploy stage
#
FROM gcr.io/distroless/cc-debian11

ARG ARCH=aarch64

COPY --from=build /usr/local/cargo/bin/mcl /usr/local/bin/mcl
COPY --from=build /usr/src/mcl/.env /.env


CMD ["mcl"]


