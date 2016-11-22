/* functions for interacting with the client */
use std::net::{TcpStream};


/*
 * gathers the top 30 posts in the client's area and their replies into a
 * buffer then sends them over the TCP stream to the client in the anonymoose
 * post transmission format described in the documentation.
 */
pub fn send_posts(mut stream: TcpStream) {
	//TODO: actually implement this, for the time being send a single dummy post

	//ask for client's location
}