# AKR-TMS development commands. Everything here acts on the compose project `akr-tms` only and never
# removes a container, a volume or an image.

SHELL := /bin/bash
.SHELLFLAGS := -euo pipefail -c
.DEFAULT_GOAL := help

ROOT := $(patsubst %/,%,$(dir $(abspath $(lastword $(MAKEFILE_LIST)))))
BACKEND := $(ROOT)/backend
FRONTEND := $(ROOT)/frontend
DEV := $(ROOT)/deploy/dev
CERTS := $(DEV)/certs
SECRETS := $(DEV)/secrets

COMPOSE := docker compose --project-directory $(ROOT)
# The stores the binaries on the host use. `make up-all` starts every service of the bundle.
DEV_SERVICES := postgres valkey openbao objectstore s3guard

CARGO := cargo --locked
CTL := $(CARGO) run --quiet --manifest-path $(BACKEND)/Cargo.toml -p akr-ctl --

export AKR_ENV ?= development
export AKR_DB_HOST ?= 127.0.0.1
export AKR_DB_PORT ?= 5470
export AKR_DB_NAME ?= akr_dev
export AKR_DB_SSLROOTCERT ?= $(CERTS)/ca.crt
export AKR_SECRETS_DIR ?= $(SECRETS)
export AKR_ZONES ?= $(ROOT)/deploy/zones.yaml

DB_TLS := sslmode=verify-full&sslrootcert=$(CERTS)/ca.crt
db_url = postgres://$(1):$$(cat $(SECRETS)/$(2))@$(AKR_DB_HOST):$(AKR_DB_PORT)/$(3)?$(DB_TLS)

# The object store as the host sees it: the guard for the product, the store itself for the installer.
GUARD_URL := http://127.0.0.1:9471
STORE_URL := http://127.0.0.1:9470
STORE_ROOT := AKR_STORE_ACCESS_KEY_FILE=$(SECRETS)/store_root_user AKR_STORE_SECRET_KEY_FILE=$(SECRETS)/store_root_password

# A binary on the host signs in to the dev OpenBao with its own AppRole (`make bao-dev` wrote the ids).
host_env = VALKEY_URL=redis://127.0.0.1:6470 S3_ENDPOINT=$(GUARD_URL) \
	BAO_ADDR=https://127.0.0.1:8270 BAO_CACERT=$(CERTS)/ca.crt \
	BAO_ROLE_ID=$$(cat $(SECRETS)/bao_$(1)_role_id) BAO_SECRET_ID_FILE=$(SECRETS)/bao_$(1)_secret_id

.PHONY: help certs secrets up up-all stop down ps db migrate check bao-dev bao-bundle store-dev store-conformance \
	store-conformance-direct \
	zone-check zone-rules prepare prepare-check generate-api test test-all lint deny run-api run-worker \
	run-ingestd

help: ## List the targets
	@grep -hE '^[a-z-]+:.*## ' $(MAKEFILE_LIST) | awk -F ':.*## ' '{printf "  %-18s %s\n", $$1, $$2}'

certs: ## Development CA and server certificates (kept when present)
	$(DEV)/gen-certs.sh

secrets: ## Development secrets (each written only when missing)
	$(DEV)/gen-secrets.sh

up: certs secrets ## Start the stores the binaries on the host use, and wait until they are healthy
	$(COMPOSE) up -d --wait $(DEV_SERVICES)

up-all: certs secrets ## Start every service of the bundle
	$(COMPOSE) up -d --wait

stop: ## Stop this project's containers; nothing is removed
	$(COMPOSE) stop

down: ## Stop this project's containers and remove them; volumes are kept
	$(COMPOSE) down

ps: ## This project's containers
	$(COMPOSE) ps

db: ## Create the database and the login roles (akr-ctl db provision)
	$(CTL) db provision

# Provisioning runs again after the migrations: a login role can be made a member only of roles that
# exist, and the migrations create them.
migrate: ## Apply the migrations, then finish provisioning
	$(CTL) db migrate
	$(CTL) db provision

check: ## The database checks (akr-ctl db check)
	$(CTL) db check

bao-dev: ## Initialise the dev OpenBao 1-of-1, unseal it and write the dev keys, policies and secrets
	$(DEV)/bao-dev.sh

# The dev OpenBao holds one view of the database. `bao-dev` writes the host's (127.0.0.1:5470), for
# binaries started with `make run-*`; this writes the view from inside the zone networks, for the
# binaries started as containers.
bao-bundle: ## As bao-dev, with the database address the containers use
	AKR_DEV_DB_HOST=postgres AKR_DEV_DB_PORT=5432 AKR_DEV_DB_CA=/run/akr/tls/ca.crt $(DEV)/bao-dev.sh

store-dev: ## Dev store users with the shipped policies, then the buckets (akr-ctl store init)
	$(DEV)/store-dev.sh
	S3_ENDPOINT=$(STORE_URL) $(STORE_ROOT) $(CTL) store init

# The suite creates and locks a bucket of its own, so it runs with the installer's credential, and the
# guard lets it through only from the installer's address (AKR_S3GUARD_INSTALLER_CIDRS in dev.env).
store-conformance: ## The WORM conformance suite, through the guard
	S3_ENDPOINT=$(GUARD_URL) $(STORE_ROOT) $(CTL) store conformance --through-guard

store-conformance-direct: ## The same suite against the store itself, with no guard in front
	S3_ENDPOINT=$(STORE_URL) $(STORE_ROOT) $(CTL) store conformance

zone-check: ## Probe from every service's position and compare with deploy/zones.yaml
	$(CTL) net matrix | $(ROOT)/deploy/compose/zone-check.sh

zone-rules: ## Print the host rules that keep the CBS reachable from akr-ingestd only
	$(CTL) net render

prepare: ## Write backend/.sqlx from the migrated dev database
	cd $(BACKEND) && DATABASE_URL="$(call db_url,akr_migrate,db_akr_migrate,$(AKR_DB_NAME))" \
		cargo sqlx prepare --workspace -- --all-targets

prepare-check: ## Fail when backend/.sqlx is not what the code needs
	cd $(BACKEND) && DATABASE_URL="$(call db_url,akr_migrate,db_akr_migrate,$(AKR_DB_NAME))" \
		cargo sqlx prepare --workspace --check -- --all-targets

generate-api: ## Write frontend/openapi.json from the API, then the typed client
	$(CARGO) run --quiet --manifest-path $(BACKEND)/Cargo.toml -p akr-api -- openapi > $(FRONTEND)/openapi.json.new
	mv $(FRONTEND)/openapi.json.new $(FRONTEND)/openapi.json
	pnpm --dir $(FRONTEND) run generate-api

test: ## The tests that need PostgreSQL only
	cd $(BACKEND) && SQLX_OFFLINE=true DATABASE_URL="$(call db_url,postgres,pg_superuser,postgres)" \
		$(CARGO) test --workspace

test-all: ## Every test, including the ones that need Valkey and the object store
	cd $(BACKEND) && SQLX_OFFLINE=true DATABASE_URL="$(call db_url,postgres,pg_superuser,postgres)" \
		$(CARGO) test --workspace -- --include-ignored

lint: ## Format check and clippy with warnings as errors
	cd $(BACKEND) && cargo fmt --all --check
	cd $(BACKEND) && SQLX_OFFLINE=true $(CARGO) clippy --workspace --all-targets -- -D warnings

deny: ## Licences, advisories and banned crates (cargo-deny)
	cd $(BACKEND) && cargo deny check

run-api: ## akr-api on 127.0.0.1:8080, metrics on 9101
	cd $(BACKEND) && AKR_BIND=127.0.0.1:8080 AKR_METRICS_BIND=127.0.0.1:9101 \
		$(call host_env,akr-api) $(CARGO) run -p akr-api

run-worker: ## akr-worker on 127.0.0.1:8081, metrics on 9102
	cd $(BACKEND) && AKR_BIND=127.0.0.1:8081 AKR_METRICS_BIND=127.0.0.1:9102 \
		$(call host_env,akr-worker) $(CARGO) run -p akr-worker

run-ingestd: ## akr-ingestd on 127.0.0.1:8082, metrics on 9103
	cd $(BACKEND) && AKR_BIND=127.0.0.1:8082 AKR_METRICS_BIND=127.0.0.1:9103 \
		$(call host_env,akr-ingestd) $(CARGO) run -p akr-ingestd
