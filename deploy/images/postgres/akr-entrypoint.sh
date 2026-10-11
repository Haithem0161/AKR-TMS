#!/bin/sh
# PostgreSQL refuses a private key that its own user does not own or that others can read, and a mounted
# secret is owned by the host. The container runs as postgres, so a copy it makes is its own.
set -eu

TLS_SRC="${AKR_PG_TLS_DIR:-/run/akr/tls}"
TLS_DST=/var/lib/postgresql/tls

if [ -f "$TLS_SRC/postgres.crt" ] && [ -f "$TLS_SRC/postgres.key" ]; then
    install -m 0644 "$TLS_SRC/postgres.crt" "$TLS_DST/server.crt"
    install -m 0600 "$TLS_SRC/postgres.key" "$TLS_DST/server.key"
fi

exec docker-entrypoint.sh "$@"
