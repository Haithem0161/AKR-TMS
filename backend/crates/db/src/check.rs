//! What `akr-ctl db check` reads from an installed database: the text round trip, the display collations and
//! the privileges the roles hold. Every function reads; none changes the installation.
//!
//! These statements read PostgreSQL's own catalogue, not a table of a migration, so they are checked when
//! they run and not at build time.

use sqlx::{Connection, PgConnection};

/// Synthetic text in the three scripts the banks write in, with the marks that break a wrong encoding:
/// combining vowel signs, Arabic-Indic and Persian digits, the letters Kurdish and Persian add, and a
/// zero-width non-joiner.
pub const TEXT_FIXTURE: [(&str, &str); 3] = [
    ("arabic", "مُؤسَّسة الاختبار، فرع ١٢٣"),
    ("kurdish", "بانکی تاقیکردنەوە، لقی ڕۆژهەڵات ڤ ێ ۆ"),
    ("persian", "بانک آزمایشی؛ می\u{200c}خواهم گچپژ ۱۲۳"),
];

/// Out of order on purpose. Sorted right, numbers order by value and the Kurdish and Persian letters fall
/// among the Arabic ones.
pub const SORT_FIXTURE: [&str; 7] = ["حساب 10", "ڕ", "حساب 2", "ت", "پ", "ب", "ئ"];
pub const SORT_EXPECTED: [&str; 7] = ["ئ", "ب", "پ", "ت", "حساب 2", "حساب 10", "ڕ"];

/// The display collations of migration 0001 as `(name, locale)`. PostgreSQL stores the canonical form of the
/// language tag the migration gives (`ar-u-kn-true` is kept as `ar-u-kn`).
pub const DISPLAY_COLLATIONS: [(&str, &str); 2] = [("akr_ar", "ar-u-kn"), ("akr_en", "en-u-kn")];
/// `pg_collation.collprovider` of ICU.
pub const ICU_PROVIDER: &str = "i";

pub const WRITE_PRIVILEGES: [&str; 6] = [
    "INSERT",
    "UPDATE",
    "DELETE",
    "TRUNCATE",
    "REFERENCES",
    "TRIGGER",
];
pub const ALL_PRIVILEGES: [&str; 7] = [
    "SELECT",
    "INSERT",
    "UPDATE",
    "DELETE",
    "TRUNCATE",
    "REFERENCES",
    "TRIGGER",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundTrip {
    /// One line per string that came back changed. Empty when every string survived.
    pub mismatches: Vec<String>,
    /// `false` for a role whose sessions are read only: its strings travelled as parameters only.
    pub through_table: bool,
}

fn compare(
    script: &str,
    path: &str,
    sent: &str,
    back: &str,
    chars: i32,
    bytes: i32,
) -> Option<String> {
    let sent_chars = sent.chars().count();
    if back != sent {
        Some(format!(
            "{script} ({path}): the text came back changed ({} bytes sent, {} back)",
            sent.len(),
            back.len()
        ))
    } else if usize::try_from(chars).ok() != Some(sent_chars)
        || usize::try_from(bytes).ok() != Some(sent.len())
    {
        Some(format!(
            "{script} ({path}): the server counts {chars} characters in {bytes} bytes, the client {sent_chars} in {}",
            sent.len()
        ))
    } else {
        None
    }
}

/// Sends every fixture string as a parameter and, where the session may write, through a temporary table,
/// and compares what comes back byte for byte and by the server's own count of characters.
pub async fn text_round_trip(conn: &mut PgConnection) -> Result<RoundTrip, sqlx::Error> {
    const MEASURE: &str = "SELECT $1::text, char_length($1::text), octet_length($1::text)";
    let mut mismatches = Vec::new();

    for (script, text) in TEXT_FIXTURE {
        let (back, chars, bytes): (String, i32, i32) = sqlx::query_as(MEASURE)
            .bind(text)
            .fetch_one(&mut *conn)
            .await?;
        mismatches.extend(compare(script, "parameter", text, &back, chars, bytes));
    }

    let read_only: String =
        sqlx::query_scalar("SELECT current_setting('default_transaction_read_only')")
            .fetch_one(&mut *conn)
            .await?;
    let through_table = read_only != "on";
    if through_table {
        let mut tx = conn.begin().await?;
        sqlx::raw_sql(
            "CREATE TEMP TABLE akr_text_check (script text PRIMARY KEY, value text NOT NULL) ON COMMIT DROP",
        )
        .execute(&mut *tx)
        .await?;
        for (script, text) in TEXT_FIXTURE {
            sqlx::query("INSERT INTO akr_text_check (script, value) VALUES ($1, $2)")
                .bind(script)
                .bind(text)
                .execute(&mut *tx)
                .await?;
        }
        for (script, text) in TEXT_FIXTURE {
            let (back, chars, bytes): (String, i32, i32) = sqlx::query_as(
                "SELECT value, char_length(value), octet_length(value) FROM akr_text_check WHERE script = $1",
            )
            .bind(script)
            .fetch_one(&mut *tx)
            .await?;
            mismatches.extend(compare(script, "table", text, &back, chars, bytes));
        }
        tx.rollback().await?;
    }

    Ok(RoundTrip {
        mismatches,
        through_table,
    })
}

/// [`SORT_FIXTURE`] in the order of the display collation `admin.akr_ar`.
pub async fn arabic_sort(conn: &mut PgConnection) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT x FROM unnest($1::text[]) AS t(x) ORDER BY x COLLATE admin.akr_ar")
        .bind(&SORT_FIXTURE[..])
        .fetch_all(conn)
        .await
}

/// The collations of the `admin` schema as `(name, provider, locale)`.
pub async fn display_collations(
    conn: &mut PgConnection,
) -> Result<Vec<(String, String, Option<String>)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT c.collname::text, c.collprovider::text, c.colllocale
         FROM pg_collation c JOIN pg_namespace n ON n.oid = c.collnamespace
         WHERE n.nspname = 'admin' ORDER BY 1",
    )
    .fetch_all(conn)
    .await
}

/// The indexes of `schemas` built on an ICU collation. There must be none: the display collations may
/// change with an ICU upgrade, and an index built on one would then be corrupt.
pub async fn indexes_on_icu_collations(
    conn: &mut PgConnection,
    schemas: &[&str],
) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT DISTINCT n.nspname || '.' || c.relname
         FROM pg_index i
         JOIN pg_class c ON c.oid = i.indexrelid
         JOIN pg_namespace n ON n.oid = c.relnamespace
         CROSS JOIN LATERAL unnest(i.indcollation::oid[]) AS used(oid)
         JOIN pg_collation co ON co.oid = used.oid
         WHERE n.nspname = ANY($1) AND co.collprovider = 'i'
         ORDER BY 1",
    )
    .bind(schemas)
    .fetch_all(conn)
    .await
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleAttributes {
    pub name: String,
    pub can_login: bool,
    pub superuser: bool,
    pub create_db: bool,
    pub create_role: bool,
    pub bypass_rls: bool,
    pub replication: bool,
    /// `pg_roles.rolconfig`: the settings every session of the role starts with.
    pub config: Vec<String>,
    pub member_of: Vec<String>,
}

type RoleRow = (
    String,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    Vec<String>,
    Vec<String>,
);

pub async fn role_attributes(
    conn: &mut PgConnection,
    role: &str,
) -> Result<Option<RoleAttributes>, sqlx::Error> {
    let row: Option<RoleRow> = sqlx::query_as(
        "SELECT r.rolname::text, r.rolcanlogin, r.rolsuper, r.rolcreatedb, r.rolcreaterole, r.rolbypassrls,
                r.rolreplication, COALESCE(r.rolconfig, '{}')::text[],
                ARRAY(SELECT g.rolname::text FROM pg_auth_members m JOIN pg_roles g ON g.oid = m.roleid
                      WHERE m.member = r.oid ORDER BY 1)
         FROM pg_roles r WHERE r.rolname = $1",
    )
    .bind(role)
    .fetch_optional(conn)
    .await?;
    Ok(row.map(
        |(
            name,
            can_login,
            superuser,
            create_db,
            create_role,
            bypass_rls,
            replication,
            config,
            member_of,
        )| {
            RoleAttributes {
                name,
                can_login,
                superuser,
                create_db,
                create_role,
                bypass_rls,
                replication,
                config,
                member_of,
            }
        },
    ))
}

/// `schema.table:PRIVILEGE` for each of `privileges` that `role` holds, itself or through a role it is a
/// member of, on a table of `schemas`. With `tables`, only those tables are looked at.
pub async fn table_privileges(
    conn: &mut PgConnection,
    role: &str,
    schemas: &[&str],
    tables: Option<&[&str]>,
    privileges: &[&str],
) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT n.nspname || '.' || c.relname || ':' || p.privilege
         FROM pg_class c
         JOIN pg_namespace n ON n.oid = c.relnamespace
         CROSS JOIN unnest($4::text[]) AS p(privilege)
         WHERE n.nspname = ANY($2)
           AND c.relkind IN ('r', 'p', 'v', 'm', 'f')
           AND ($3::text[] IS NULL OR c.relname = ANY($3))
           AND has_table_privilege($1, c.oid, p.privilege)
         ORDER BY 1",
    )
    .bind(role)
    .bind(schemas)
    .bind(tables)
    .bind(privileges)
    .fetch_all(conn)
    .await
}

/// The schemas of `schemas` that `role` may look into.
pub async fn schema_usage(
    conn: &mut PgConnection,
    role: &str,
    schemas: &[&str],
) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT n.nspname::text FROM pg_namespace n
         WHERE n.nspname = ANY($2) AND has_schema_privilege($1, n.oid, 'USAGE')
         ORDER BY 1",
    )
    .bind(role)
    .bind(schemas)
    .fetch_all(conn)
    .await
}

/// Every privilege granted to PUBLIC on a schema of `schemas` (`schema:PRIVILEGE`) or on a relation in one
/// (`schema.relation:PRIVILEGE`).
pub async fn public_grants(
    conn: &mut PgConnection,
    schemas: &[&str],
) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT grant_line FROM (
             SELECT n.nspname || '.' || c.relname || ':' || a.privilege_type AS grant_line
             FROM pg_class c
             JOIN pg_namespace n ON n.oid = c.relnamespace
             CROSS JOIN LATERAL aclexplode(c.relacl) AS a
             WHERE n.nspname = ANY($1) AND a.grantee = 0
             UNION ALL
             SELECT n.nspname || ':' || a.privilege_type
             FROM pg_namespace n
             CROSS JOIN LATERAL aclexplode(n.nspacl) AS a
             WHERE n.nspname = ANY($1) AND a.grantee = 0
         ) AS grants
         ORDER BY 1",
    )
    .bind(schemas)
    .fetch_all(conn)
    .await
}

/// The tables of `schemas` that `owner` does not own.
pub async fn tables_not_owned_by(
    conn: &mut PgConnection,
    schemas: &[&str],
    owner: &str,
) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT n.nspname || '.' || c.relname || ' (' || pg_get_userbyid(c.relowner) || ')'
         FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
         WHERE n.nspname = ANY($1) AND c.relkind IN ('r', 'p') AND pg_get_userbyid(c.relowner) <> $2
         ORDER BY 1",
    )
    .bind(schemas)
    .bind(owner)
    .fetch_all(conn)
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fixture_holds_what_a_wrong_encoding_breaks() {
        let all: String = TEXT_FIXTURE.iter().map(|(_, text)| *text).collect();
        for needed in [
            '\u{064f}', '\u{0651}', '\u{0661}', '\u{06f1}', '\u{200c}', 'پ', 'چ', 'ژ', 'گ', 'ڕ',
            'ڵ', 'ڤ', 'ێ', 'ۆ', 'ە',
        ] {
            assert!(all.contains(needed), "U+{:04X} is missing", needed as u32);
        }
        assert!(TEXT_FIXTURE.iter().all(|(_, text)| !text.is_ascii()));
    }

    #[test]
    fn the_sort_fixture_and_its_expected_order_hold_the_same_strings() {
        let mut given = SORT_FIXTURE.to_vec();
        let mut expected = SORT_EXPECTED.to_vec();
        assert_ne!(given, expected);
        given.sort_unstable();
        expected.sort_unstable();
        assert_eq!(given, expected);
    }

    #[test]
    fn a_changed_or_miscounted_string_is_a_mismatch() {
        let text = TEXT_FIXTURE[0].1;
        let chars = i32::try_from(text.chars().count()).unwrap_or(0);
        let bytes = i32::try_from(text.len()).unwrap_or(0);
        assert_eq!(
            compare("arabic", "parameter", text, text, chars, bytes),
            None
        );
        assert!(compare("arabic", "parameter", text, "?????", chars, bytes).is_some());
        assert!(compare("arabic", "table", text, text, bytes, bytes).is_some());
    }
}
