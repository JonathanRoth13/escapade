FROM rust:1.87-bookworm AS build

WORKDIR /src
COPY . .
RUN cargo build --release


FROM debian:bookworm-slim AS tablebase

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl zstd && \
    rm -rf /var/lib/apt/lists/*

ARG TABLEBASE_URL="https://github.com/JonathanRoth13/escapade/releases/download/v1.0.0-alpha/escapade-tablebase.tar.zst"
ARG TABLEBASE_SHA256="6a5054401c5c61c656e4940e606b2340b4899f53dbb05c93cbbd2882263a629d"


WORKDIR /opt/escapade/tablebase

RUN curl -fL "$TABLEBASE_URL" -o tablebase.tar.zst && \
    echo "$TABLEBASE_SHA256  tablebase.tar.zst" | sha256sum -c - && \
    zstd -d tablebase.tar.zst -c | tar -xf - -C /opt/escapade/tablebase


# ---------- Runtime ----------
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir -p /opt/escapade/bin /opt/escapade/tablebase

COPY --from=build /src/target/release/escapade /opt/escapade/bin/escapade

COPY --from=tablebase /opt/escapade/tablebase /opt/escapade/tablebase

ENV PATH="/opt/escapade/bin:${PATH}"
ENV TABLEBASE_DIR=/opt/escapade/tablebase

EXPOSE 8080

ENTRYPOINT ["escapade"]
CMD ["engine"]
