#!/usr/bin/env bash
# Development only: gives the dev store its two service users with the shipped policies, using the `mc`
# inside the dev store's container. An install creates them with the installer credential, and the
# product never uses `mc`.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SECRETS="$ROOT/deploy/dev/secrets"
POLICIES="$ROOT/deploy/compose/config/objectstore"

store() { docker compose --project-directory "$ROOT" exec -T objectstore "$@"; }
mc() { store mc --config-dir /tmp/mc "$@"; }

mc alias set dev http://127.0.0.1:9000 "$(cat "$SECRETS/store_root_user")" \
    "$(cat "$SECRETS/store_root_password")" >/dev/null

for user in writer presign; do
    store sh -c "cat > /tmp/policy-$user.json" < "$POLICIES/policy-$user.json"
    mc admin policy create dev "akr-$user" "/tmp/policy-$user.json" >/dev/null
    mc admin user add dev "$(cat "$SECRETS/s3_${user}_access_key")" "$(cat "$SECRETS/s3_${user}_secret_key")" \
        >/dev/null
    mc admin policy attach dev "akr-$user" --user "$(cat "$SECRETS/s3_${user}_access_key")" >/dev/null 2>&1 ||
        true
    echo "store user $(cat "$SECRETS/s3_${user}_access_key"): policy akr-$user"
done
