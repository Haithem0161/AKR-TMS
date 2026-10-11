-- migration 0003_platform_idempotency_settings_flags.sql
-- Phase 00. Idempotency ledger (DBS migrations 0001, 0003 and 0024 folded in from the start, DR-21), runtime
-- settings and feature flags.

CREATE TABLE IF NOT EXISTS admin.idempotency_keys (
    principal_id          UUID NOT NULL,       -- FK to admin.users added by phase 01 (deferred FK)
    key                   TEXT NOT NULL CHECK (key ~ '^[ -~]{8,255}$'),   -- 8-255 printable ASCII (API.md s.1)
    endpoint              TEXT NOT NULL,       -- method + route template, for the cockpit and the TTL report
    request_hash          BYTEA NOT NULL CHECK (octet_length(request_hash) = 32),   -- raw SHA-256 bytes
    response_status       INT,
    response_content_type TEXT,                -- NULL for an empty body (204)
    response_body         BYTEA,               -- AES-256-GCM sealed under the `idempotency` key family
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at          TIMESTAMPTZ,         -- NULL = in flight
    PRIMARY KEY (principal_id, key),
    CHECK ((completed_at IS NULL) = (response_status IS NULL))
);
CREATE INDEX IF NOT EXISTS idx_idem_created ON admin.idempotency_keys (created_at);

-- The registry of data keys (DR-424): one row per key family and month. `wrapped` is the 256-bit key as OpenBao
-- transit ciphertext under akr-kek; the key itself exists only in process memory. Every sealed value names its
-- row by id, so a row is never changed or deleted while a value sealed under it exists.
CREATE TABLE IF NOT EXISTS admin.data_keys (
    id          UUID PRIMARY KEY,      -- UUIDv7, app-side; carried by every value sealed under the key
    family      TEXT NOT NULL CHECK (family ~ '^[a-z][a-z0-9-]*$'),
    period      TEXT NOT NULL CHECK (period ~ '^[0-9]{4}-(0[1-9]|1[0-2])$'),   -- the UTC month the key seals in
    wrapped     TEXT NOT NULL CHECK (wrapped ~ '^vault:v[0-9]+:'),             -- transit ciphertext, never the key
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (family, period)
);

CREATE TABLE IF NOT EXISTS admin.settings (
    key         TEXT PRIMARY KEY CHECK (key ~ '^[a-z][a-z0-9_]*(\.[a-z0-9_]+)+$'),
    value       JSONB NOT NULL,
    value_type  TEXT NOT NULL CHECK (value_type IN ('integer', 'decimal', 'boolean', 'string', 'duration_s',
                                                    'cron', 'timestamp', 'json')),
    floor       JSONB,                 -- regulatory limit {"min": n} | {"max": n} | both, plus "basis"; NULL = none
    governed    BOOLEAN NOT NULL DEFAULT TRUE,   -- TRUE: changes only through a change request (phase 04)
    label       JSONB NOT NULL CHECK (label ? 'ar' AND label ? 'en'),
    updated_by  UUID,                  -- FK to admin.users added by phase 01 (deferred FK); NULL = seeded
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    version     INT NOT NULL DEFAULT 1,
    CHECK (floor IS NULL OR governed)
);

CREATE TABLE IF NOT EXISTS admin.feature_flags (
    key         TEXT PRIMARY KEY CHECK (key ~ '^[a-z][a-z0-9_]*(\.[a-z0-9_]+)+$'),
    enabled     BOOLEAN NOT NULL DEFAULT FALSE,
    scope       JSONB NOT NULL DEFAULT '{}'::jsonb,   -- {"branches": [uuid], "roles": [code]}; {} = everyone
    label       JSONB NOT NULL CHECK (label ? 'ar' AND label ? 'en'),
    updated_by  UUID,                  -- FK to admin.users added by phase 01 (deferred FK)
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    version     INT NOT NULL DEFAULT 1
);

-- The idempotency ledger holds copies of mutation responses: application role only, never akr_readonly
-- (DBS migration 0019 lesson). Settings and flags are read by every service role.
GRANT SELECT, INSERT, UPDATE, DELETE ON admin.idempotency_keys TO akr_app;
GRANT SELECT, INSERT ON admin.data_keys TO akr_app;   -- wrapped keys only; no UPDATE, no DELETE
GRANT SELECT, UPDATE ON admin.settings, admin.feature_flags TO akr_app;   -- rows are seeded by migrations only
GRANT SELECT ON admin.settings, admin.feature_flags TO akr_ingest;

-- Phase 00 keys. Every key the code reads is declared once in common::settings_registry; a test asserts that the
-- registry and these rows agree. Values are the planning defaults; the bank changes them through A2 (phase 03/04).
INSERT INTO admin.settings (key, value, value_type, floor, governed, label) VALUES
 ('locale.default',                 '"ar"',             'string',     NULL, FALSE,
  '{"ar": "اللغة الافتراضية للواجهة", "en": "Default interface language"}'),
 ('bank.timezone',                  '"Asia/Baghdad"',   'string',     NULL, TRUE,
  '{"ar": "المنطقة الزمنية للمصرف", "en": "Bank time zone"}'),
 ('idempotency.ttl_days',           '7',                'integer',    NULL, FALSE,
  '{"ar": "مدة الاحتفاظ بمفاتيح منع التكرار (أيام)", "en": "Idempotency key retention (days)"}'),
 ('idempotency.in_flight_lease_s',  '120',              'duration_s', NULL, FALSE,
  '{"ar": "مهلة الطلب قيد التنفيذ (ثوانٍ)", "en": "In-flight request lease (seconds)"}'),
 ('jobs.lease_s',                   '600',              'duration_s', NULL, FALSE,
  '{"ar": "مهلة حجز المهمة الخلفية (ثوانٍ)", "en": "Background job lease (seconds)"}'),
 ('jobs.done_retention_days',       '30',               'integer',    NULL, FALSE,
  '{"ar": "مدة الاحتفاظ بالمهام المنجزة (أيام)", "en": "Finished job retention (days)"}'),
 ('outbox.retention_days',          '30',               'integer',    NULL, FALSE,
  '{"ar": "مدة الاحتفاظ برسائل الصادر المرسلة (أيام)", "en": "Dispatched outbox retention (days)"}'),
 ('outbox.max_attempts',            '10',               'integer',    NULL, FALSE,
  '{"ar": "الحد الأقصى لمحاولات إرسال رسالة الصادر", "en": "Outbox delivery attempts"}'),
 ('timers.sweep_interval_s',        '30',               'duration_s', NULL, FALSE,
  '{"ar": "فاصل فحص المؤقتات (ثوانٍ)", "en": "Timer sweep interval (seconds)"}'),
 ('timers.terminal_retention_days', '90',               'integer',    NULL, FALSE,
  '{"ar": "مدة الاحتفاظ بالمؤقتات المنتهية (أيام)", "en": "Fired or cancelled timer retention (days)"}'),
 ('schedule.maintenance_sweep',     '"0 30 2 * * *"',   'cron',       NULL, FALSE,
  '{"ar": "موعد الصيانة اليومية", "en": "Daily maintenance time"}'),
 ('schedule.backup_check',          '"0 0 6 * * *"',    'cron',       NULL, FALSE,
  '{"ar": "موعد فحص النسخ الاحتياطي", "en": "Backup check time"}'),
 ('ops.backup_marker_path',         '"/var/lib/akr/backup/heartbeat"', 'string', NULL, TRUE,
  '{"ar": "مسار علامة النسخ الاحتياطي", "en": "Backup heartbeat marker path"}'),
 ('ops.backup_max_age_h',           '26',               'integer',    NULL, TRUE,
  '{"ar": "أقصى عمر مقبول لآخر نسخة احتياطية (ساعات)", "en": "Maximum backup age (hours)"}'),
 ('ops.last_restore_drill_at',      'null',             'timestamp',  NULL, TRUE,
  '{"ar": "تاريخ آخر تمرين استعادة ناجح", "en": "Last successful restore drill"}'),
 ('ops.restore_drill_max_age_days', '31',               'integer',    NULL, TRUE,
  '{"ar": "أقصى مدة بين تمارين الاستعادة (أيام)", "en": "Maximum days between restore drills"}')
ON CONFLICT (key) DO NOTHING;

-- A test fixture for FlagsRepository. NOT the security boundary of the dev-only /_echo route: that route is
-- mounted only when AKR_ENV != production (DBS lesson), so flipping this row in production exposes nothing.
INSERT INTO admin.feature_flags (key, enabled, label) VALUES
 ('dev.echo_route', FALSE, '{"ar": "مسار الاختبار للمطورين", "en": "Developer echo route"}')
ON CONFLICT (key) DO NOTHING;
