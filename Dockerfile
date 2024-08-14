FROM docker.io/paritytech/ci-unified:latest as builder

WORKDIR /acala
COPY . .

RUN cargo fetch
RUN cargo build --locked --release

# =============

FROM docker.io/parity/base-bin:latest
LABEL maintainer="hello@acala.network"

COPY --from=builder /acala/target/release/acala /usr/local/bin

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /acala acala && \
	mkdir -p /acala/data /acala/.local/share && \
	chown -R acala:acala /acala/data && \
	ln -s /acala/data /acala/.local/share/acala && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/acala --version

USER acala

EXPOSE 30333 9933 9944 9615
VOLUME ["/acala/data"]

ENTRYPOINT ["/usr/local/bin/acala"]
