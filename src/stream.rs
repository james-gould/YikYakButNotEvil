/* functions for interacting with the client */
use std::net::{TcpStream};
use std::io::*;

use post::*;

/* tells the client the server is terminating the session */
pub fn terminate(mut stream: &TcpStream) {
	let mut writer = BufWriter::new(&mut stream);
	match writer.write("203\n".as_bytes()) {
		Ok(_) => return (),
		Err(e) => println!("Termination failed with error {}", e),
	}
}

/* tells the client the server is ready for IO */
pub fn ready(mut stream: &TcpStream) {
	let mut writer = BufWriter::new(&mut stream);
	match writer.write("201\n".as_bytes()) {
		Ok(_) => return (),
		Err(e) => println!("Ready status update failed with error {}", e),
	}
}

/* tells the client the server completed the operation successfully */
pub fn success(mut stream: &TcpStream) {
	let mut writer = BufWriter::new(&mut stream);
	match writer.write("200\n".as_bytes()) {
		Ok(_) => return (),
		Err(e) => println!("Success status updated failed with error {}", e),
	}
}

/* sends a vector of bytes to the client */
pub fn send_to_client(mut stream: &TcpStream, payload: Vec<u8>)
{
		let mut writer = BufWriter::new(&mut stream);
		for i in payload {
	   		writer.write(&[i]).unwrap();
		}
		match writer.write("\n".as_bytes()) {
	   		Ok(_) => return (),
	   		Err(e) => println!("Write failed with error {}", e),
		}

}

/* receives a vector of bytes from the client */
pub fn recieve_from_client(mut stream: &TcpStream) -> Vec<u8>
{
	/* tell the client we're ready for IO */
	ready(stream);

	let endchar: u8 = 0x0a;

	/* read the stream into a buffer */
	let mut reader = BufReader::new(&mut stream);
	let mut in_buffer: Vec<u8> = Vec::new();
	reader.read_until(endchar, &mut in_buffer);
	 	
	return in_buffer;
}

/* gets information about the user for further processing */
pub fn initial_connection(mut stream: &TcpStream) -> User
{
	/* tell the client we're ready for the user data */
	ready(stream);

	let endchar: u8 = 0x0a;

	/* read the stream into a buffer */
	let mut reader = BufReader::new(&mut stream);
	let mut in_buffer: Vec<u8> = Vec::new();
	reader.read_until(endchar, &mut in_buffer);

	return user_decode(in_buffer);
}





