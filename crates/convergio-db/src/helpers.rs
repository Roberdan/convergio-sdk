//! Query helpers and schema introspection.

use rusqlite::Connection;

/// Check if a table exists in the database.
pub fn table_exists(conn: &Connection, table: &str) -> rusqlite::Result<bool> {
    conn.query_row(
        "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name=?1",
        [table],
        |r| r.get(0),
    )
}

/// Check if a column exists in a table.
pub fn column_exists(conn: &Connection, table: &str, column: &str) -> rusqlite::Result<bool> {
    let columns = get_column_names(conn, table)?;
    Ok(columns.iter().any(|c| c == column))
}

/// Get all column names for a table.
///
/// The table name is validated to contain only safe identifier characters
/// before interpolation into the PRAGMA statement (SQLite PRAGMAs do not
/// support parameterised arguments).
pub fn get_column_names(conn: &Connection, table: &str) -> rusqlite::Result<Vec<String>> {
    if !is_safe_identifier(table) {
        return Err(rusqlite::Error::InvalidParameterName(format!(
            "unsafe table name: {table}"
        )));
    }
    let mut stmt = conn.prepare(&format!("PRAGMA table_info('{table}')"))?;
    let names = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(names)
}

/// Returns `true` if `name` contains only alphanumeric chars and underscores
/// (safe for use as a SQL identifier).
fn is_safe_identifier(name: &str) -> bool {
    !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

/// Check if an index exists.
pub fn index_exists(conn: &Connection, index: &str) -> rusqlite::Result<bool> {
    conn.query_row(
        "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='index' AND name=?1",
        [index],
        |r| r.get(0),
    )
}

/// Returns true if the error is SQLITE_BUSY or SQLITE_LOCKED.
pub fn is_busy_error(e: &rusqlite::Error) -> bool {
    match e {
        rusqlite::Error::SqliteFailure(err, _) => {
            matches!(
                err.code,
                rusqlite::ffi::ErrorCode::DatabaseBusy | rusqlite::ffi::ErrorCode::DatabaseLocked
            )
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("CREATE TABLE example (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")
            .unwrap();
        conn
    }

    #[test]
    fn table_exists_true() {
        let conn = test_conn();
        assert!(table_exists(&conn, "example").unwrap());
    }

    #[test]
    fn table_exists_false() {
        let conn = test_conn();
        assert!(!table_exists(&conn, "nonexistent").unwrap());
    }

    #[test]
    fn column_exists_true() {
        let conn = test_conn();
        assert!(column_exists(&conn, "example", "name").unwrap());
    }

    #[test]
    fn column_exists_false() {
        let conn = test_conn();
        assert!(!column_exists(&conn, "example", "missing").unwrap());
    }

    #[test]
    fn get_columns() {
        let conn = test_conn();
        let cols = get_column_names(&conn, "example").unwrap();
        assert_eq!(cols, vec!["id", "name", "age"]);
    }

    #[test]
    fn rejects_sql_injection_in_table_name() {
        let conn = test_conn();
        assert!(get_column_names(&conn, "'; DROP TABLE example; --").is_err());
        assert!(get_column_names(&conn, "foo bar").is_err());
        assert!(get_column_names(&conn, "").is_err());
    }

    #[test]
    fn safe_identifier_check() {
        assert!(is_safe_identifier("my_table"));
        assert!(is_safe_identifier("table123"));
        assert!(!is_safe_identifier(""));
        assert!(!is_safe_identifier("table name"));
        assert!(!is_safe_identifier("table;drop"));
    }
}
