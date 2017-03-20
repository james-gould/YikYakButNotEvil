/* functions for interacting with the client */
use std::net::{TcpStream};
use std::io::*;

use post::*;

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
	let mut reader = BufReader::new(&mut stream);
	
	/* currently reserves memory for the longest possible post length,
	TODO optimise this */
	let mut buffer: [u8; 1082] = [0; 1082];
	reader.read(&mut buffer);
	let vector = buffer.to_vec();
	 	
	return vector;
}

/* gets information about the user for further processing */
pub fn initial_connection(stream: &TcpStream) -> User
{
	let buffer: Vec<u8> = recieve_from_client(stream);
	return user_decode(buffer);
}





