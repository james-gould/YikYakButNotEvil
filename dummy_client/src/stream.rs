/* functions for interacting with the server */
use std::net::{TcpStream};
use std::io::*;

use post::*;

/* sends a vector of bytes to the client */
pub fn send_to_server(mut stream: &TcpStream, payload: Vec<u8>)
{
	   let mut writer = BufWriter::new(&mut stream);
	   for i in payload {
	   		writer.write(&[i]).unwrap();
	   }

}

/* receives a vector of bytes from the client */
pub fn recieve_from_server(mut stream: &TcpStream) -> Vec<u8>
{
	let mut reader = BufReader::new(&mut stream);
	let mut buffer: Vec<u8> = Vec::new();
	reader.read_to_end(&mut buffer);
	 	
	return buffer;
}






