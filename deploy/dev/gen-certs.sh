#!/bin/sh
# Development only. Writes a throwaway CA and one server certificate per TLS service into
# deploy/dev/certs/ (git-ignored). A bank install takes its certificates from the bank's CA.
set -eu

DIR="$(cd "$(dirname "$0")" && pwd)/certs"
mkdir -p "$DIR"
cd "$DIR"

if [ ! -f ca.key ]; then
    openssl ecparam -name prime256v1 -genkey -noout -out ca.key
    openssl req -x509 -new -key ca.key -sha256 -days 3650 -subj "/CN=AKR-TMS development CA" -out ca.crt
fi

issue() {
    name="$1"
    [ -f "$name.crt" ] && return 0
    openssl ecparam -name prime256v1 -genkey -noout -out "$name.key"
    openssl req -new -key "$name.key" -subj "/CN=$name" -out "$name.csr"
    printf 'subjectAltName=DNS:%s,DNS:localhost,IP:127.0.0.1\nextendedKeyUsage=serverAuth\n' "$name" > "$name.ext"
    openssl x509 -req -in "$name.csr" -CA ca.crt -CAkey ca.key -CAcreateserial -sha256 -days 825 \
        -extfile "$name.ext" -out "$name.crt"
    rm -f "$name.csr" "$name.ext"
    chmod 0644 "$name.key"
}

issue postgres
issue openbao
issue edge
echo "certificates in $DIR"
