#!/usr/bin/env bash
# Development only: initialises the dev OpenBao with one unseal key and gives it the layout of the plan
# (transit keys, the KV mount `akr/`, one AppRole per binary with its boot and run policies).
#
# A bank install does this in the unseal ceremony (Shamir 3-of-5, SP-12) and never with this script: the
# root token and the unseal key written here to deploy/dev/secrets/ would defeat the whole design.
#
# Run it again at any time: it unseals a sealed dev OpenBao and adds only what is missing.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SECRETS="$ROOT/deploy/dev/secrets"
INIT="$SECRETS/bao_init.json"
umask 077

# Where the binaries find the database: on loopback when they run on the host with `cargo run`.
DB_HOST="${AKR_DEV_DB_HOST:-127.0.0.1}"
DB_PORT="${AKR_DEV_DB_PORT:-5470}"
DB_NAME="${AKR_DEV_DB_NAME:-akr_dev}"
DB_CA="${AKR_DEV_DB_CA:-$ROOT/deploy/dev/certs/ca.crt}"

TOKEN=""
bao() {
    docker compose --project-directory "$ROOT" exec -T -e "BAO_TOKEN=$TOKEN" openbao bao "$@"
}

status=0
bao operator init -status >/dev/null 2>&1 || status=$?
if [ "$status" -eq 2 ]; then
    bao operator init -key-shares=1 -key-threshold=1 -format=json > "$INIT"
    echo "initialised: one unseal key, in $INIT"
elif [ ! -s "$INIT" ]; then
    echo "the dev OpenBao is initialised and $INIT is missing: its keys are gone" >&2
    exit 1
fi
TOKEN="$(jq -r .root_token "$INIT")"

if [ "$( (bao status -format=json || true) | jq -r .sealed)" = "true" ]; then
    bao operator unseal "$(jq -r '.unseal_keys_b64[0]' "$INIT")" >/dev/null
    echo "unsealed"
fi
for _ in $(seq 1 60); do
    bao token lookup >/dev/null 2>&1 && break
    timeout 0.5 tail -f /dev/null || true
done
bao token lookup >/dev/null

mounted() { bao "$1" list -format=json | jq -e --arg path "$2/" 'has($path)' >/dev/null; }
mounted secrets transit || bao secrets enable transit >/dev/null
mounted secrets akr || bao secrets enable -path=akr -version=2 kv >/dev/null
mounted auth approle || bao auth enable approle >/dev/null

transit_key() {
    bao read "transit/keys/$1" >/dev/null 2>&1 || bao write -f "transit/keys/$1" "type=$2" >/dev/null
}
transit_key akr-kek aes256-gcm96
# akr-sign-decision is reserved and unused (DR-294); akr-sign-bundle is used under an operator policy.
for key in akr-sign-anchor akr-sign-package akr-sign-certificate akr-sign-deletion akr-sign-bundle akr-sign-decision; do
    transit_key "$key" ed25519
done

kv_read() { printf 'path "akr/data/%s" { capabilities = ["read"] }\n' "$@"; }
transit() {
    op="$1"; shift
    printf 'path "transit/'"$op"'/%s" { capabilities = ["update"] }\n' "$@"
}
SIGNING="akr-sign-anchor akr-sign-package akr-sign-certificate akr-sign-deletion"

boot_policy() {
    case "$1" in
        akr-api) kv_read db/akr_app s3/presign 'keys/*' ;;
        akr-worker) kv_read db/akr_app s3/writer 'keys/*' ;;
        akr-ingestd) kv_read db/akr_ingest 'sources/*' ;;
        akr-assess) kv_read db/akr_assess ;;
    esac
    printf 'path "auth/token/create/%s-run" { capabilities = ["update"] }\n' "$1"
}

run_policy() {
    case "$1" in
        akr-api) transit decrypt akr-kek ;;
        akr-worker)
            transit encrypt akr-kek
            transit decrypt akr-kek
            transit datakey/wrapped akr-kek
            # shellcheck disable=SC2086
            transit sign $SIGNING
            # shellcheck disable=SC2086
            transit verify $SIGNING
            for key in $SIGNING; do
                printf 'path "transit/keys/%s" { capabilities = ["read"] }\n' "$key"
            done
            ;;
    esac
    printf 'path "auth/token/renew-self" { capabilities = ["update"] }\n'
    printf 'path "sys/seal-status" { capabilities = ["read"] }\n'
    printf 'path "sys/health" { capabilities = ["read"] }\n'
}

for binary in akr-api akr-worker akr-ingestd akr-assess; do
    boot_policy "$binary" | bao policy write "$binary-boot" - >/dev/null
    run_policy "$binary" | bao policy write "$binary-run" - >/dev/null
    # Orphan: the run token outlives the login token it was created with, which the binary revokes.
    bao write "auth/token/roles/$binary-run" "allowed_policies=$binary-run" orphan=true renewable=true \
        token_period=1h >/dev/null
    bao write "auth/approle/role/$binary" "token_policies=$binary-boot" token_ttl=10m token_max_ttl=30m \
        >/dev/null
    bao read -field=role_id "auth/approle/role/$binary/role-id" > "$SECRETS/bao_${binary}_role_id"
    if [ ! -s "$SECRETS/bao_${binary}_secret_id" ]; then
        bao write -f -field=secret_id "auth/approle/role/$binary/secret-id" > "$SECRETS/bao_${binary}_secret_id"
    fi
    # A binary run as a container reads its ids as the image's user, not as the one who ran this.
    chmod 0644 "$SECRETS/bao_${binary}_role_id" "$SECRETS/bao_${binary}_secret_id"
done

kv_put() { bao kv put -mount=akr "$1" - >/dev/null; }

for role in akr_migrate akr_app akr_ingest akr_assess akr_readonly akr_monitor akr_retention_login; do
    [ -s "$SECRETS/db_$role" ] || continue
    jq -n --arg host "$DB_HOST" --argjson port "$DB_PORT" --arg database "$DB_NAME" --arg username "$role" \
        --rawfile password "$SECRETS/db_$role" --arg ca "$DB_CA" \
        '{host: $host, port: $port, database: $database, username: $username,
          password: ($password | rtrimstr("\n")), sslmode: "verify-full", sslrootcert: $ca}' |
        kv_put "db/$role"
done

for user in writer presign; do
    [ -s "$SECRETS/s3_${user}_secret_key" ] || continue
    jq -n --rawfile id "$SECRETS/s3_${user}_access_key" --rawfile secret "$SECRETS/s3_${user}_secret_key" \
        '{access_key_id: ($id | rtrimstr("\n")), secret_access_key: ($secret | rtrimstr("\n"))}' |
        kv_put "s3/$user"
done

for name in cursor-hmac match-key; do
    bao kv get -mount=akr "keys/$name" >/dev/null 2>&1 && continue
    jq -n --arg key "$(openssl rand -base64 32)" '{key: $key}' | kv_put "keys/$name"
done

echo "dev OpenBao ready: https://127.0.0.1:8270"
echo "  transit   akr-kek and six signing keys"
echo "  KV akr/   $(bao kv list -mount=akr -format=json db | jq -r 'map("db/" + .) | join(" ")') s3/* keys/*"
echo "  AppRoles  akr-api akr-worker akr-ingestd akr-assess (role and secret ids in $SECRETS)"
