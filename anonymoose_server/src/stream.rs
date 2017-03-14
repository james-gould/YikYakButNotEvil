/* functions for interacting with the client */
use std::net::{TcpStream};
use std::io::*;
use post;
use post::Post;

/* sends a post to the client */
pub fn send_post_to_client(mut stream: &TcpStream, payload: Vec<u8>)
{
	   let mut writer = BufWriter::new(&mut stream);
	   for i in payload {
	   		writer.write(&[i]).unwrap();
	   }

}