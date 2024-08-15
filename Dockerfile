FROM docker.io/paritytech/ci-unified:latest AS builder

WORKDIR /acala
COPY . .

RUN cargo fetch
RUN cargo build --locked --release

# =============

FROM phusion/baseimage:focal-1.2.0
LABEL maintainer="hello@acala.network"

RUN useradd -m -u 1000 -U -s /bin/sh -d /acala acala

COPY --from=builder /acala/target/release/acala /usr/local/bin

# checks
RUN ldd /usr/local/bin/acala && \
	/usr/local/bin/acala --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
	rm -rf /usr/sbin /usr/share/man

USER acala

EXPOSE 30333 9933 9615

RUN mkdir /acala/data

VOLUME ["/acala/data"]

ENTRYPOINT ["/usr/local/bin/acala"]