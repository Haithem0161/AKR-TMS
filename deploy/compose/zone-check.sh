#!/usr/bin/env bash
# Probes the running stack from every service's position and compares what it finds with the matrix that
# `akr-ctl net matrix` prints from deploy/zones.yaml (read here on standard input).
#
#   akr-ctl net matrix --zones deploy/zones.yaml | deploy/compose/zone-check.sh
#
# For each service, one short-lived container joins exactly that service's networks and tries every
# service port in the stack, by name and by every address the target holds. An `open` row must connect
# and a `closed` row must not. A `zone` row is a pair that shares a network without an edge: compose
# cannot separate it (the cluster profile allows each edge alone), so it is counted and not judged. A
# row whose target is not running is skipped and named in the last line.
set -euo pipefail

PROJECT="${AKR_COMPOSE_PROJECT:-akr-tms}"
IMAGE="${AKR_PROBE_IMAGE:-akr/postgres:17}"
CONNECT_TIMEOUT_S="${AKR_PROBE_TIMEOUT_S:-2}"

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

cat > "$WORK/matrix"
if [ ! -s "$WORK/matrix" ]; then
    echo "zone-check: no matrix on standard input (akr-ctl net matrix)" >&2
    exit 1
fi

mine=(--filter "label=com.docker.compose.project=$PROJECT")
docker network ls "${mine[@]}" --format '{{.Name}}' > "$WORK/networks"
docker ps "${mine[@]}" --filter status=running --format '{{.Label "com.docker.compose.service"}}' |
    sort -u > "$WORK/running"

while read -r service; do
    held="$(docker ps -q "${mine[@]}" --filter "label=com.docker.compose.service=$service" --filter status=running |
        xargs docker inspect --format '{{range .NetworkSettings.Networks}}{{.IPAddress}} {{end}}' | tr '\n' ' ')"
    echo "$service $held"
done < "$WORK/running" > "$WORK/addresses"

read -r -d '' PROBE <<'EOF' || true
while read -r to port targets; do
    for target in "$to" $targets; do
        (
            if timeout "$CONNECT_TIMEOUT_S" bash -c "exec 3<>/dev/tcp/$target/$port" 2>/dev/null; then
                echo "$to $port open"
            else
                echo "$to $port closed"
            fi
        ) &
    done
done
wait
EOF

: > "$WORK/failures"
: > "$WORK/totals"
: > "$WORK/absent"
printf '%-18s %8s %10s %10s %12s %8s %5s\n' position "open ok" "closed ok" "zone open" "zone closed" skipped FAIL

for from in $(cut -f1 "$WORK/matrix" | sort -u); do
    attach=()
    IFS=, read -ra names <<< "$(awk -F'\t' -v from="$from" '$1 == from { print $4; exit }' "$WORK/matrix")"
    for name in "${names[@]}"; do
        if grep -qx "${PROJECT}_$name" "$WORK/networks"; then
            attach+=(--network "${PROJECT}_$name")
        else
            echo "$name" >> "$WORK/absent"
        fi
    done
    [ "${#attach[@]}" -gt 0 ] || attach=(--network none)

    awk -F'\t' -v from="$from" '
        FILENAME == ARGV[1] { service = $0; sub(/ .*/, "", service); held = $0; sub(/^[^ ]+ ?/, "", held)
                              addresses[service] = held; next }
        $1 == from && ($2 in addresses) { print $2, $3, addresses[$2] }
    ' "$WORK/addresses" "$WORK/matrix" > "$WORK/targets"

    : > "$WORK/results"
    if [ -s "$WORK/targets" ]; then
        docker run --rm -i --name "$PROJECT-zone-probe-$from" "${attach[@]}" \
            --read-only --cap-drop ALL --security-opt no-new-privileges:true --user 65534:65534 \
            --env "CONNECT_TIMEOUT_S=$CONNECT_TIMEOUT_S" --entrypoint bash "$IMAGE" -c "$PROBE" \
            < "$WORK/targets" > "$WORK/results"
    fi

    awk -F'\t' -v from="$from" -v failures="$WORK/failures" -v totals="$WORK/totals" '
        FILENAME == ARGV[1] { split($0, row, " "); key = row[1] ":" row[2]; seen[key] = 1
                              if (row[3] == "open") reached[key] = 1; next }
        $1 != from { next }
        {
            key = $2 ":" $3
            if (!(key in seen)) { skipped++; not_running[$2] = 1; next }
            got = (key in reached) ? "open" : "closed"
            if ($5 == "zone") { if (got == "open") zone_open++; else zone_closed++ }
            else if ($5 == got) { if (got == "open") open_ok++; else closed_ok++ }
            else { failed++; printf "FAIL  %s -> %s:%s  expected %s, got %s\n", from, $2, $3, $5, got >> failures }
        }
        END {
            printf "%-18s %8d %10d %10d %12d %8d %5d\n", from, open_ok, closed_ok, zone_open, zone_closed, skipped, failed
            printf "%d %d %d %d\n", open_ok + closed_ok, failed, zone_open + zone_closed, skipped >> totals
            for (service in not_running) print service >> (totals ".absent")
        }
    ' "$WORK/results" "$WORK/matrix"
done

read -r judged failed zone skipped <<< "$(awk '{ a += $1; b += $2; c += $3; d += $4 } END { print a + 0, b + 0, c + 0, d + 0 }' "$WORK/totals")"

echo
cat "$WORK/failures"
echo "zone-check: $judged rows match the matrix, $failed do not."
echo "  $zone rows are pairs on one network with no edge between them: the compose profile separates by"
echo "  network, the cluster profile by edge."
if [ "$skipped" -gt 0 ]; then
    echo "  $skipped rows skipped, their target is not running:" \
        "$(sort -u "$WORK/totals.absent" | tr '\n' ' ')"
fi
if [ -s "$WORK/absent" ]; then
    echo "  networks not created yet, no running service is on them:" "$(sort -u "$WORK/absent" | tr '\n' ' ')"
fi
[ "$failed" -eq 0 ]
