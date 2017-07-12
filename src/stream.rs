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

/* sends an error code to the client */
pub fn error(mut stream: &TcpStream, code: i16)
{
	let code_vec = code.to_string().into_bytes();
	send_to_client(&stream, code_vec);
	
}

/* sends a vector of bytes to the client */
pub fn send_to_client(mut stream: &TcpStream, payload: Vec<u8>)
{
		let mut writer = BufWriter::new(&mut stream);
		for i in payload {
	   		match writer.write(&[i]) {
	   			Ok(_) => (),
	   			Err(e) => println!("Write failed with error {}", e),
			}
		}
		match writer.write("\n".as_bytes()) {
	   		Ok(_) => (),
	   		Err(e) => println!("Write failed with error {}", e),
		}

}

/* receives a vector of bytes from the client */
pub fn recieve_from_client(mut stream: &TcpStream) -> Option<Vec<u8>>
{
	/* tell the client we're ready for IO */
	ready(stream);

	let endchar: u8 = 0x0a;

	/* read the stream into a buffer */
	let mut reader = BufReader::new(&mut stream);
	let mut in_buffer: Vec<u8> = Vec::new();
	match reader.read_until(endchar, &mut in_buffer) {
		Ok(_) => return Some(in_buffer),
		Err(e) => {
			println!("Read failed with error {}.", e);
			return None;
		}
	}

}

/* same as the above, but without sending the ready */
pub fn recieve_from_client_quiet(mut stream: &TcpStream) -> Option<Vec<u8>>
{
	let endchar: u8 = 0x0a;

	/* read the stream into a buffer */
	let mut reader = BufReader::new(&mut stream);
	let mut in_buffer: Vec<u8> = Vec::new();
	match reader.read_until(endchar, &mut in_buffer) {
		Ok(_) => return Some(in_buffer),
		Err(e) => {
			println!("Read failed with error {}.", e);
			return None;
		}
	}

}

/* gets information about the user for further processing */
pub fn initial_connection(mut stream: &TcpStream) -> Option<User>
{
	/* tell the client we're ready for the user data */
	ready(stream);

	let endchar: u8 = 0x0a;

	/* read the stream into a buffer */
	let mut reader = BufReader::new(&mut stream);
	let mut in_buffer: Vec<u8> = Vec::new();
	match reader.read_until(endchar, &mut in_buffer) {
		Ok(_) => println!("\n"),
		Err(e) => {
			println!("Read failed with error {}.", e);
			return None;
		}
	}
	match user_decode(in_buffer) {
		Ok(u) => return Some(u),
		Err(e) => { 
			println!("Error parsing user data, {}", e);
			return None;
		}
	}
	
}





