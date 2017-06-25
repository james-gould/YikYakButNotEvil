/* functions for interacting with the server */
use std::net::{TcpStream};
use std::io::*;

use post::*;

/* sends 100 to the server */
pub fn send_100(mut stream: &TcpStream)
{
	let mut writer = BufWriter::new(&mut stream);
	writer.write("100\n".as_bytes());
}

/* sends a vector of bytes to the server */
pub fn send_to_server(mut stream: &TcpStream, payload: Vec<u8>)
{
	   let mut writer = BufWriter::new(&mut stream);
	   let mut counter: i32 = 0;
	   for i in payload {
	   		writer.write(&[i]).unwrap();
	   		counter = counter + 1;
	   }
	   println!("wrote {} bytes", counter);
	   writer.write("\n".as_bytes());

}

/* receives a vector of bytes from the server */
pub fn recieve_from_server(mut stream: &TcpStream) -> Vec<u8>
{
	let mut reader = BufReader::new(&mut stream);
	let mut buffer: Vec<u8> = Vec::new();
	reader.read_to_end(&mut buffer);
	 	
	return buffer;
}






