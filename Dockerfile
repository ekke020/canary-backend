FROM rust:alpine3.16 as build

WORKDIR /build

COPY . .
RUN cargo build -r
# RUN cargo install --path .

FROM alpine

USER nobody:nobody
COPY --from=build /build/target/release/canary-backend /canary-backend
COPY --from=build /build/data/ /data

CMD [ "/canary-backend" ]