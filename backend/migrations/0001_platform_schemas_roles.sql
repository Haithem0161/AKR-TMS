-- migration 0001_platform_schemas_roles.sql
-- Phase 00. Schemas, NOLOGIN roles and schema usage. No table grants here: every table is granted in the
-- migration that creates it (no ALTER DEFAULT PRIVILEGES, DATA-MODEL.md section 21).
-- Roles: EXCEPTION WHEN duplicate_object, not IF NOT EXISTS: two concurrent migrations (sqlx::test gives every
-- test its own database) can both pass an existence check. LOGIN, passwords and role settings are provisioning
-- (DR-22; DBS migrations 0019/0020 lesson: concurrent ALTER ROLE fails with "tuple concurrently updated").

DO $$ BEGIN CREATE ROLE akr_app NOLOGIN;          EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE akr_ingest NOLOGIN;       EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE akr_assess NOLOGIN;       EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE akr_audit_writer NOLOGIN; EXCEPTION WHEN duplicate_object THEN NULL; END $$;
DO $$ BEGIN CREATE ROLE akr_readonly NOLOGIN;     EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE SCHEMA IF NOT EXISTS admin;
CREATE SCHEMA IF NOT EXISTS audit;
CREATE SCHEMA IF NOT EXISTS raw;
CREATE SCHEMA IF NOT EXISTS canon;
CREATE SCHEMA IF NOT EXISTS feat;
CREATE SCHEMA IF NOT EXISTS scn;
CREATE SCHEMA IF NOT EXISTS alert;
CREATE SCHEMA IF NOT EXISTS casework;   -- not `case`: CASE is reserved (DR-20)
CREATE SCHEMA IF NOT EXISTS kyc;
CREATE SCHEMA IF NOT EXISTS ref;
CREATE SCHEMA IF NOT EXISTS rpt;
CREATE SCHEMA IF NOT EXISTS assess;

REVOKE ALL ON SCHEMA public FROM PUBLIC;

GRANT USAGE ON SCHEMA admin, ref, feat, scn, alert, casework, kyc, rpt, canon TO akr_app;
GRANT USAGE ON SCHEMA admin, raw, canon                                      TO akr_ingest;
GRANT USAGE ON SCHEMA admin, assess, ref, feat                               TO akr_assess;
GRANT USAGE ON SCHEMA audit                                                  TO akr_audit_writer;
GRANT USAGE ON SCHEMA admin, audit, casework, alert, rpt, kyc, scn, ref      TO akr_readonly;

-- Readiness compares the embedded migrator with the applied chain, so every service role reads the ledger.
-- Guarded: under sqlx::test the ledger is created in the test database's default schema instead.
DO $$ BEGIN
    IF to_regclass('admin._sqlx_migrations') IS NOT NULL THEN
        GRANT SELECT ON admin._sqlx_migrations TO akr_app, akr_ingest, akr_assess;
    END IF;
END $$;

-- Display collations only. No index may use them (ICU version changes then cost nothing); the database default
-- stays builtin C.UTF-8. Used as ORDER BY ... COLLATE akr_ar on name lists (UX parity test with Intl.Collator,
-- research.md RQ-10).
CREATE COLLATION IF NOT EXISTS admin.akr_ar (provider = icu, locale = 'ar-u-kn-true');
CREATE COLLATION IF NOT EXISTS admin.akr_en (provider = icu, locale = 'en-u-kn-true');
