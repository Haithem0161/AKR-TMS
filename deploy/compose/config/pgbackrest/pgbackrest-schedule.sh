#!/bin/sh
# The backup schedule of profile S: a full backup every Sunday, a differential every other night, four
# fulls kept (pgbackrest.conf). WAL is archived by the server itself, at most 60 seconds behind.
#
# After every backup that succeeds, the marker that `backup_check` reads is touched. A backup that fails
# leaves the marker alone, so its age keeps growing and the alert fires.
set -eu

MARKER="${AKR_BACKUP_MARKER:-/var/lib/akr/backup/heartbeat}"
HOUR="${AKR_BACKUP_HOUR:-01}"
STANZA=akr

PGBACKREST_REPO1_CIPHER_PASS="$(cat "$AKR_BACKUP_PASSPHRASE_FILE")"
export PGBACKREST_REPO1_CIPHER_PASS

pgbackrest --stanza="$STANZA" stanza-create
pgbackrest --stanza="$STANZA" check

last=""
while true; do
    today="$(date -u +%F)"
    if [ "$(date -u +%H)" = "$HOUR" ] && [ "$today" != "$last" ]; then
        if [ "$(date -u +%u)" = 7 ]; then kind=full; else kind=diff; fi
        if pgbackrest --stanza="$STANZA" --type="$kind" backup; then
            touch "$MARKER"
            last="$today"
        else
            echo "backup of type $kind failed; it is tried again in a minute" >&2
        fi
    fi
    sleep 60
done
