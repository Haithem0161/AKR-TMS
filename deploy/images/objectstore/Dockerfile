# The AKR object store image: the Silo source at the pinned commit, compiled here, on a distroless base
# as the store's own user. No console, no update check, no shell.
ARG GO_IMAGE
ARG BASE_IMAGE=gcr.io/distroless/cc-debian12:nonroot@sha256:9dac0a79194e45a7da0158a9c6da57b217585af0786db3845d1f0ec1a0dd182f

FROM ${GO_IMAGE} AS build
WORKDIR /src
COPY source/ .
ARG SILO_TAG
RUN CGO_ENABLED=0 go build -trimpath -buildvcs=false \
        -ldflags "-s -w -X github.com/minio/minio/cmd.ReleaseTag=${SILO_TAG}" -o /out/minio .

FROM ${BASE_IMAGE}
COPY --from=build /out/minio /usr/local/bin/minio
USER 10001:10001
EXPOSE 9000
ENTRYPOINT ["/usr/local/bin/minio"]
