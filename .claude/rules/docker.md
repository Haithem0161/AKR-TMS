---
paths:
  - "docker-compose.yml"
  - "deploy/**"
  - "Makefile"
  - "**/.env*"
---

# Docker Rules

Ported from the digital-banking-system repository (`.claude/rules/docker.md`, read 2026-10-11) and adapted to
AKR-TMS. Differences from the DBS rule are marked "AKR".

**Docker provides the stores. In development the binaries and the frontend run on the host.** AKR: the same compose
file is the single-server deployment profile, so every binary can also run as a container, and `make up-all` starts
the whole bundle.

## Architecture (AKR)

- One compose project, `akr-tms`. The root `docker-compose.yml` includes `deploy/compose/compose.yaml` and the
  development overlay `deploy/compose/compose.dev.yaml` with `deploy/compose/dev.env`.
- `make up` starts the five development stores and waits until they are healthy: PostgreSQL 17, Valkey, OpenBao, the
  object store, and `akr-s3guard` in front of it.
- The binaries run on the host with `make run-api`, `make run-worker` and `make run-ingestd`. The frontend runs with
  `pnpm --dir frontend dev`.
- **Valkey is cache, rate limits and SSE fan-out only**, never a durable queue. Durable work is a row in the
  PostgreSQL `jobs` table (see `jobs.md`).
- **Zone networks.** Every service is on the zone networks `deploy/zones.yaml` gives it and on no other. A service is
  added to `deploy/zones.yaml` in the same change that adds it to the compose file, and `make zone-check` must pass
  afterwards. The development overlay adds one network, `dev_host`, which publishes the stores on loopback; it is not
  part of a deployment.
- **Profiles.** A service that cannot start without something a later phase or the bank supplies is behind a compose
  profile, so `up` never starts it half-configured: `identity` (Kratos and Hydra, from phase 01), `phase2`
  (`akr-assess`), `backup`, `siem`, `standby`.
- **Hardening.** Every service is read-only, drops every capability, sets `no-new-privileges` and runs as a user that
  is not root. An exception is written beside the service with its reason.
- **Images are pinned by digest** in the compose file and listed in `deploy/images.lock`
  (`tools/ci/check_images_lock.py` compares the two). A tag alone is never a pin.

## Allowed Commands

- DO: `make up`, `make up-all`, `make stop`, `make down`, `make ps` (each acts on the project `akr-tms` only)
- DO: `docker compose up -d`, `docker compose stop`, `docker compose restart`, `docker compose logs`
- DO: `docker logs`, `docker exec`, `docker ps`, `docker inspect`
- DO: update the compose files, `deploy/zones.yaml`, `deploy/images.lock` and the Dockerfiles under `deploy/images/`

## FORBIDDEN Commands (NEVER run these)

- **NEVER** run `docker rm`, `docker compose rm`, or any command that removes containers by name or id
- **NEVER** run `docker system prune`, `docker container prune`, `docker volume prune`, `docker image prune` or
  `docker network prune`
- **NEVER** run `docker compose down -v` unless the user asked for the data to be destroyed, in so many words
- AKR: **NEVER** stop, restart or change a container that does not carry the label
  `com.docker.compose.project=akr-tms`. The development machine runs other projects' containers.
- AKR: the one removal in use is the `--rm` flag on a short-lived probe container that the same command starts
  (`deploy/compose/zone-check.sh`).

## Container Management

```bash
make up                 # the five stores, waits for healthy
make bao-dev            # initialise and unseal the dev OpenBao, write keys, policies and secrets (host view)
make db && make migrate # database and login roles, then the migrations
make store-dev          # dev store users and the buckets
make ps                 # this project's containers
make stop               # stop; nothing is removed
make down               # stop and remove this project's containers; volumes are kept
```

OpenBao is sealed after its container restarts: run `make bao-dev` again. The dev OpenBao holds one view of the
database: `make bao-dev` writes the host's (`127.0.0.1:5470`), `make bao-bundle` the view from inside the zone
networks (`postgres:5432`), which the binaries need when they run as containers.

## Connection Details (development, AKR)

Host ports are non-standard on purpose: the standard ones are taken on the development machine. Every published port
is on loopback only.

| Service | Host address | Container port | Notes |
|---|---|---|---|
| PostgreSQL | `127.0.0.1:5470` | 5432 | TLS required (`sslmode=verify-full`, CA `deploy/dev/certs/ca.crt`); database `akr_dev` |
| Valkey | `127.0.0.1:6470` | 6379 | |
| OpenBao | `https://127.0.0.1:8270` | 8200 | TLS with the dev CA |
| Object store | `http://127.0.0.1:9470` | 9000 | the installer only; the product never talks to it directly |
| `akr-s3guard` | `http://127.0.0.1:9471` | 9000 | what the product uses as `S3_ENDPOINT` |
| Edge (Caddy) | `https://localhost:8443` | 443 | the certificate is chosen by name: use `localhost`, not `127.0.0.1` |

Passwords are files under `deploy/dev/secrets/` (`make secrets`), never values in a compose file, an env file or a
command line. A binary reads its database credential from OpenBao, never from `DATABASE_URL`; `DATABASE_URL` exists
only for `cargo sqlx prepare` and for the tests.

## Decision Trees

**A store is not starting:**
1. Check the host port: `ss -ltnp | grep -E '5470|6470|8270|9470|9471'`
2. Check the logs: `docker compose logs --tail 50 <service>`
3. Check health: `make ps`
4. On a port conflict: change the host port in `deploy/compose/compose.dev.yaml` and in this table, the Makefile and
   `deploy/dev/` in the same change

**A binary cannot start as a container:**
1. `docker compose logs --tail 20 <service>`; the binary prints the reason on its last line
2. "sealed" or a 503 from OpenBao: `make bao-bundle`
3. A database host that cannot be resolved or reached: the KV holds the host view; `make bao-bundle`

**After changing a compose file:** `docker compose config --quiet`, then `make up` (compose recreates only what
changed), then `make zone-check`. A volume change that would destroy data needs the user's explicit approval.

**Schema out of sync:**
1. `make up`, then `make migrate`
2. `make prepare` after any change to SQL inside a `query!` macro; `make prepare-check` must pass before a commit

## Images (AKR)

- `deploy/images/akr/Dockerfile`: one build stage, one target per binary, distroless runtime, user 65532. No shell in
  the image: each long-running binary is its own health probe (`<binary> healthcheck`).
- Only release binaries go into an image, never source code. `cargo auditable` embeds the dependency list.
- Nothing in an image or a compose file reaches outside the installation: no update check, no telemetry, no external
  AI API. An outbound address is a declared entry in `deploy/zones.yaml`.
