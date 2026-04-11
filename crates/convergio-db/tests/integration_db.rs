//! Integration tests for convergio-db.
//! Tests pool creation, migrations, and helpers as an external consumer.

use convergio_db::migration::{apply_migrations, current_version, ensure_registry};
use convergio_db::pool::create_memory_pool;
use convergio_types::extension::Migration;

#[test]
fn memory_pool_creates_and_queries() {
    let pool = create_memory_pool().expect("pool should create");
    let conn = pool.get().expect("should get connection");

    conn.execute_batch(
        "CREATE TABLE test_items (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
         INSERT INTO test_items (name) VALUES ('alpha');
         INSERT INTO test_items (name) VALUES ('beta');",
    )
    .unwrap();

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM test_items", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 2);
}

#[test]
fn migration_creates_registry_and_applies() {
    let pool = create_memory_pool().unwrap();
    let conn = pool.get().unwrap();

    ensure_registry(&conn).unwrap();

    let migrations = &[Migration {
        version: 1,
        description: "create widgets",
        up: "CREATE TABLE widgets (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
    }];

    apply_migrations(&conn, "test_module", migrations).unwrap();

    let ver = current_version(&conn, "test_module").unwrap();
    assert_eq!(ver, 1);

    // Table should exist
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE name = 'widgets'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert!(exists);
}
