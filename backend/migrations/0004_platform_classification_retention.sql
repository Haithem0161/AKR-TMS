-- migration 0004_platform_classification_retention.sql
-- Phase 00. Column classification (drives masking, field encryption and policy) and retention assignment (read by
-- maintenance_sweep now and the retention engine in phase 23). Every later migration inserts rows for the tables
-- it creates; the coverage tests in section 6 fail the build otherwise.

DO $$ BEGIN
    CREATE TYPE admin.data_classification AS ENUM
        ('public', 'internal', 'personal', 'financial', 'detection-logic', 'secret-str', 'card');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

CREATE TABLE IF NOT EXISTS admin.column_classification (
    schema_name    TEXT NOT NULL,
    table_name     TEXT NOT NULL,
    column_name    TEXT NOT NULL,
    classification admin.data_classification NOT NULL,
    encrypted      BOOLEAN NOT NULL DEFAULT FALSE,   -- `enc`: AES-256-GCM ciphertext under an OpenBao-wrapped DEK
    keyed_hash     BOOLEAN NOT NULL DEFAULT FALSE,   -- `hk`: HMAC-SHA256 twin under the match key
    note           TEXT,
    PRIMARY KEY (schema_name, table_name, column_name),
    CHECK (NOT encrypted OR classification IN ('personal', 'financial', 'secret-str', 'card'))
);

CREATE TABLE IF NOT EXISTS admin.retention_assignment (
    schema_name     TEXT NOT NULL,
    table_name      TEXT NOT NULL,
    retention_class TEXT NOT NULL CHECK (retention_class ~ '^R([1-9]|1[0-3])$'),
    clock           TEXT NOT NULL CHECK (clock IN ('partition', 'age', 'relationship_end', 'closure', 'decision',
                                                   'retirement', 'owner', 'never')),
    clock_column    TEXT,            -- the column the clock reads; NULL for retirement, owner and never
    note            TEXT,
    PRIMARY KEY (schema_name, table_name),
    CHECK ((retention_class = 'R10') = (clock = 'never')),
    CHECK ((clock IN ('partition', 'age', 'relationship_end', 'closure', 'decision')) = (clock_column IS NOT NULL))
);

GRANT SELECT ON admin.column_classification, admin.retention_assignment TO akr_app, akr_ingest, akr_readonly;
-- Every binary loads the classification register at start, to keep classified values out of its logs.
GRANT SELECT ON admin.column_classification TO akr_assess;

-- The two non-internal columns of this phase first, then the rest as internal.
INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification, encrypted, note)
VALUES ('admin', 'idempotency_keys', 'response_body', 'secret-str', TRUE,
        'Copy of a mutation response; may carry suspicion-tier content'),
       ('admin', 'data_keys', 'wrapped', 'secret-str', FALSE,
        'A data key wrapped by the OpenBao KEK (transit ciphertext); never serialised')
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.column_classification (schema_name, table_name, column_name, classification)
SELECT 'admin', v.t, c, 'internal'
FROM (VALUES
  ('jobs',                  ARRAY['id','kind','payload','status','run_at','attempts','max_attempts','last_error',
                                  'locked_at','locked_by','coalesce_key','created_at','finished_at']),
  ('outbox',                ARRAY['id','topic','payload','created_at','available_at','dispatched_at','attempts',
                                  'last_error']),
  ('timers',                ARRAY['id','owner_kind','owner_id','kind','fire_at','calendar_id','payload',
                                  'created_at','fired_at','cancelled_at']),
  ('idempotency_keys',      ARRAY['principal_id','key','endpoint','request_hash','response_status',
                                  'response_content_type','created_at','completed_at']),
  ('data_keys',             ARRAY['id','family','period','created_at']),
  ('settings',              ARRAY['key','value','value_type','floor','governed','label','updated_by','updated_at',
                                  'version']),
  ('feature_flags',         ARRAY['key','enabled','scope','label','updated_by','updated_at','version']),
  ('column_classification', ARRAY['schema_name','table_name','column_name','classification','encrypted',
                                  'keyed_hash','note']),
  ('retention_assignment',  ARRAY['schema_name','table_name','retention_class','clock','clock_column','note'])
) AS v(t, cols), unnest(v.cols) AS c
ON CONFLICT (schema_name, table_name, column_name) DO NOTHING;

INSERT INTO admin.retention_assignment (schema_name, table_name, retention_class, clock, clock_column, note) VALUES
 ('admin', 'jobs',                  'R12', 'age',        'finished_at',  'Only done and dead rows; jobs.done_retention_days'),
 ('admin', 'outbox',                'R12', 'age',        'dispatched_at','Only dispatched rows; an undispatched row is evidence that someone was not told'),
 ('admin', 'timers',                'R12', 'age',        'fired_at',     'Only fired or cancelled rows (cancelled_at for cancelled); timers.terminal_retention_days'),
 ('admin', 'idempotency_keys',      'R12', 'age',        'created_at',   'idempotency.ttl_days'),
 ('admin', 'data_keys',             'R9',  'retirement', NULL,           'Kept while any value sealed under the key exists'),
 ('admin', 'settings',              'R9',  'retirement', NULL,           'Current values; change history is in the audit trail (R10)'),
 ('admin', 'feature_flags',         'R9',  'retirement', NULL,           'As settings'),
 ('admin', 'column_classification', 'R9',  'retirement', NULL,           NULL),
 ('admin', 'retention_assignment',  'R9',  'retirement', NULL,           NULL)
ON CONFLICT (schema_name, table_name) DO NOTHING;
