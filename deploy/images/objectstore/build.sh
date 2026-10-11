#!/usr/bin/env bash
# Builds akr/objectstore from the Silo source named in source.lock. The source is fetched at the pinned
# commit, its archive digest is compared with the pinned one, and only then is it compiled.
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=source.lock
. "$HERE/source.lock"

for name in SILO_TAG SILO_REPOSITORY SILO_COMMIT SILO_SOURCE_SHA256 GO_IMAGE; do
    if [ -z "${!name}" ]; then
        echo "objectstore: $name is not set in deploy/images/objectstore/source.lock" >&2
        exit 1
    fi
done

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

git -c advice.detachedHead=false clone --quiet --no-checkout "$SILO_REPOSITORY" "$WORK/source"
git -C "$WORK/source" checkout --quiet "$SILO_COMMIT"
if [ "$(git -C "$WORK/source" describe --tags --exact-match)" != "$SILO_TAG" ]; then
    echo "objectstore: commit $SILO_COMMIT is not the tag $SILO_TAG" >&2
    exit 1
fi
found="$(git -C "$WORK/source" archive --format=tar "$SILO_COMMIT" | sha256sum | cut -d' ' -f1)"
if [ "$found" != "$SILO_SOURCE_SHA256" ]; then
    echo "objectstore: the source archive is $found, the lock says $SILO_SOURCE_SHA256" >&2
    exit 1
fi
rm -rf "$WORK/source/.git"

cp "$HERE/Dockerfile" "$WORK/Dockerfile"
docker build --build-arg "GO_IMAGE=$GO_IMAGE" --build-arg "SILO_TAG=$SILO_TAG" \
    --tag "akr/objectstore:$SILO_TAG" "$WORK"
echo "akr/objectstore:$SILO_TAG built from $SILO_REPOSITORY at $SILO_COMMIT"
