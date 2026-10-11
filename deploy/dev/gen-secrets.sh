#!/bin/sh
# Development only. Writes the secrets of the dev stack into deploy/dev/secrets/ (git-ignored), each one
# only when it is missing. A bank install keeps its secrets in OpenBao.
set -eu

DIR="$(cd "$(dirname "$0")" && pwd)/secrets"
mkdir -p "$DIR"
umask 077

# Passwords are printable ASCII: `akr-ctl db provision` refuses anything else.
secret() {
    [ -s "$DIR/$1" ] && return 0
    openssl rand -hex "${2:-24}" > "$DIR/$1"
}

# The store's and the database's containers read these as another user.
shared() {
    secret "$1" "${2:-24}"
    chmod 0644 "$DIR/$1"
}

shared pg_superuser 16
[ -s "$DIR/store_root_user" ] || { echo akr-store-root > "$DIR/store_root_user"; chmod 0644 "$DIR/store_root_user"; }
shared store_root_password 16

for role in akr_migrate akr_app akr_ingest akr_assess akr_readonly akr_monitor akr_retention_login; do
    secret "db_$role"
done

for user in writer presign; do
    [ -s "$DIR/s3_${user}_access_key" ] || echo "akr-$user" > "$DIR/s3_${user}_access_key"
    secret "s3_${user}_secret_key"
done
echo "secrets in $DIR"
