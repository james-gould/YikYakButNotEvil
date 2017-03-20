/* this module provides configuration handling for the TCP server */

pub struct Config
{
	pub listener_ip: String; /* address and port to listen on */
	pub database_ip: String; /* address and port for postgres */
	pub content_ip: String; /* address and port for content server */

	pub mode: i8; /* set mode, 0 for dev, 1 for prod ect */
	pub debug: i8; /* toggle debug messages */

}

pub fn default_conf() {
	let default = Config {
		listener_ip: "localhost:1337",
		database_ip: "postgres://postgres@localhost",
		content_ip: "localhost:1338",
		mode: 0,
		debug: 1,
	}
}