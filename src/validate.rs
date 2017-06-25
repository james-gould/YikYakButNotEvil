/* validation functions for various MooseCast tcp_server data types */
use std::net::{Shutdown, TcpStream};
use std::thread;

use post::User;
use stream::*;


/* checks whether user data send by the client exists or not */
pub fn user_data_check(stream: &TcpStream, target: Option<User>) -> User
{
	match target {
		None => {
			/* tell the client they're shit and they know they are */
			send_to_client(&stream, String::from("300\n").into_bytes());

			/* kill the socket too for good measure */
			stream.shutdown(Shutdown::Both).unwrap();
		}
		Some(_) => {
			return target.unwrap();
		}
	}
}