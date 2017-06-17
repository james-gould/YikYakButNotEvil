/* functions for interacting with the client */
use std::net::{TcpStream};
use std::io::*;

use post::*;

/* tells the client the server is ready for IO */
pub fn ready(mut stream: &TcpStream) {
	let mut writer = BufWriter::new(&mut stream);
	writer.write("201\n".as_bytes());
}

/* sends a vector of bytes to the client */
pub fn send_to_client(mut stream: &TcpStream, payload: Vec<u8>)
{
	   let mut writer = BufWriter::new(&mut stream);
	   for i in payload {
	   		writer.write(&[i]).unwrap();
	   }

}

/* receives a vector of bytes from the client */
pub fn recieve_from_client(mut stream: &TcpStream) -> Vec<u8>
{
	/* tell the client we're ready for IO */
	ready(stream);

	/* read the stream into a buffer */
	let mut reader = BufReader::new(&mut stream);
	let mut in_buffer: Vec<u8> = Vec::new();
	reader.read_to_end(&mut in_buffer);
	 	
	return in_buffer;
}

/* gets information about the user for further processing */
pub fn initial_connection(mut stream: &TcpStream) -> User
{
	let buffer: Vec<u8> = recieve_from_client(stream);
	return user_decode(buffer);
}





