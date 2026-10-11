#!/bin/sh
# A streaming replica of the primary. An empty data directory is filled with a base backup first.
set -eu

PASSWORD="$(cat "$AKR_REPLICATION_PASSWORD_FILE")"
export PGPASSWORD="$PASSWORD"
export PGSSLMODE=verify-full PGSSLROOTCERT=/run/akr/tls/ca.crt

install -m 0644 /run/akr/tls/postgres.crt /var/lib/postgresql/tls/server.crt
install -m 0600 /run/akr/tls/postgres.key /var/lib/postgresql/tls/server.key

if [ ! -s "$PGDATA/PG_VERSION" ]; then
    pg_basebackup --host="$AKR_PRIMARY_HOST" --username=akr_replication --pgdata="$PGDATA" \
        --wal-method=stream --write-recovery-conf --checkpoint=fast
    chmod 0700 "$PGDATA"
fi

exec postgres -c hba_file=/etc/postgresql/akr/pg_hba.conf -c ssl=on \
    -c ssl_cert_file=/var/lib/postgresql/tls/server.crt -c ssl_key_file=/var/lib/postgresql/tls/server.key \
    -c ssl_min_protocol_version=TLSv1.3 -c hot_standby=on
