# The AKR binaries. One stage compiles them all with the pinned toolchain and embeds the dependency list
# in each (cargo-auditable); every binary is then a target of its own (`--target akr-api`) on a
# distroless base: no shell, no package manager, a user that is not root.
#
# Build context: backend/.
ARG RUST_IMAGE=rust:1.97.1-slim-bookworm@sha256:2775a09d208ff0d7c1f50490c45b62db929e87ba1dcbc3f2132ac71a704bcdd3
ARG BASE_IMAGE=gcr.io/distroless/cc-debian12:nonroot@sha256:9dac0a79194e45a7da0158a9c6da57b217585af0786db3845d1f0ec1a0dd182f

FROM ${RUST_IMAGE} AS build
RUN cargo install --locked cargo-auditable@0.7.7
WORKDIR /src
COPY . .
# Compile-time SQL is checked against the committed .sqlx, so the build needs no database.
ENV SQLX_OFFLINE=true
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo auditable build --locked --release --bins \
    && mkdir /out \
    && for bin in akr-api akr-worker akr-ingestd akr-assess akr-s3guard akr-ctl; do \
           cp "target/release/$bin" /out/; \
       done

FROM ${BASE_IMAGE} AS runtime
USER 65532:65532
WORKDIR /

FROM runtime AS akr-api
COPY --from=build /out/akr-api /usr/local/bin/akr-api
EXPOSE 8080 9101
ENTRYPOINT ["/usr/local/bin/akr-api"]

FROM runtime AS akr-worker
COPY --from=build /out/akr-worker /usr/local/bin/akr-worker
EXPOSE 8080 9101
ENTRYPOINT ["/usr/local/bin/akr-worker"]

FROM runtime AS akr-ingestd
COPY --from=build /out/akr-ingestd /usr/local/bin/akr-ingestd
EXPOSE 8080 9101
ENTRYPOINT ["/usr/local/bin/akr-ingestd"]

FROM runtime AS akr-assess
COPY --from=build /out/akr-assess /usr/local/bin/akr-assess
EXPOSE 8080 9101
ENTRYPOINT ["/usr/local/bin/akr-assess"]

FROM runtime AS akr-s3guard
COPY --from=build /out/akr-s3guard /usr/local/bin/akr-s3guard
EXPOSE 9000 9101
ENTRYPOINT ["/usr/local/bin/akr-s3guard"]

FROM runtime AS akr-ctl
COPY --from=build /out/akr-ctl /usr/local/bin/akr-ctl
ENTRYPOINT ["/usr/local/bin/akr-ctl"]
