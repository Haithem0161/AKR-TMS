#!/usr/bin/env bash
# Lays out what a release bundle carries beside its images, and saves the signed images as one OCI
# layout. `akr-ctl bundle build` then writes the manifest over both.
#
#   stage_bundle.sh <stage directory> <image layout directory>
#
# Needs AKR_RELEASE and HARBOR (the registry the signed images were pushed to), the bills of materials
# in out/sbom, and the release's own files in release/: notes.json, notices/ and rollback/plan.md.
set -euo pipefail

STAGE="$1"
LAYOUT="$2"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
: "${AKR_RELEASE:?the release name}" "${HARBOR:?the registry of the signed images}"

for needed in release/notes.json release/notices release/rollback/plan.md out/sbom; do
    if [ ! -e "$ROOT/$needed" ]; then
        echo "stage_bundle: $needed is missing" >&2
        exit 1
    fi
done

mkdir -p "$STAGE" "$LAYOUT"
cp -r "$ROOT/backend/migrations" "$STAGE/migrations"
mkdir -p "$STAGE/deploy"
cp -r "$ROOT/deploy/compose" "$ROOT/deploy/observability" "$ROOT/deploy/zones.yaml" "$ROOT/deploy/images.lock" \
    "$STAGE/deploy/"
rm -f "$STAGE/deploy/compose/compose.dev.yaml" "$STAGE/deploy/compose/dev.env"
cp -r "$ROOT/docs" "$STAGE/docs"
cp -r "$ROOT/out/sbom" "$STAGE/sbom"
python3 "$ROOT/tools/ci/waiver_exports.py" vex "$AKR_RELEASE" "$STAGE/sbom/vex.cdx.json"
cp "$ROOT/release/notes.json" "$STAGE/notes.json"
cp -r "$ROOT/release/notices" "$STAGE/notices"
mkdir -p "$STAGE/rollback"
cp "$ROOT/release/rollback/plan.md" "$STAGE/rollback/plan.md"
if [ -d "$ROOT/out/evidence" ]; then
    cp -r "$ROOT/out/evidence" "$STAGE/evidence"
fi

# Every image of the bundle with its signature, taken back from the registry it was signed in.
while read -r image; do
    case "$image" in
        akr/*) reference="$HARBOR/$image" ;;
        *) reference="$image" ;;
    esac
    cosign save --dir "$LAYOUT" "$reference"
done < <(docker compose --project-directory "$ROOT" --profile phase2 --profile identity --profile backup \
    --profile siem -f "$ROOT/deploy/compose/compose.yaml" config --images | sort -u)
echo "staged in $STAGE; images in $LAYOUT"
