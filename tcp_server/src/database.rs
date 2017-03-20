/* provide the connection to the postgres database and utilities */
use postgres::{Connection, TlsMode};

/* this function builds a new database from scratch */
pub fn build_database() {
	/* connect to the database server */
	let conn = Connection::connect("postgres://postgres@localhost",
		TlsMode::None).unwrap();
}