#!/bin/sh
# Every 15 minutes: the object store's drive directories into the restic repository on the backup
# server. The repository encrypts what it stores; its passphrase is held off the server and mounted as a
# file. OpenBao snapshots join this schedule in phase 30, with the operator policy that may take them.
set -eu

INTERVAL_S="${AKR_RESTIC_INTERVAL_S:-900}"

restic cat config >/dev/null 2>&1 || restic init

while true; do
    restic backup --tag store --host akr /data/store || echo "restic backup failed; it is tried again" >&2
    sleep "$INTERVAL_S"
done
