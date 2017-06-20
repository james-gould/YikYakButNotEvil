/* this program implements the Anonymoose Transmission Protocol in order to test
 * functionality of the server and provide a schematic for the real clients to
 * implement the protocol correctly */
extern crate byteorder;

use std::io::*;
use std::net::{TcpStream};

mod post;
mod stream;
use post::*;
use stream::*;

fn intial_connection(stream: &TcpStream) {
    /* serialise our user data and send it to the server */
    let vector = user_encode(example_user_data());
    send_to_server(&stream, vector);
}

fn main()
{
	println!("Anonymoose Dummy Client version 0.0.1");
    /* connect to the server - the default development server uses TCP port 1337
     * on localhost although this is configurable for obvious reasons */
    println!("Connecting to server...");
    let mut stream = TcpStream::connect("localhost:1337").unwrap();

    /* the protocol dictates that "100" is sent to the server to initialise the
     * connection */
    let mut out_buffer = BufWriter::new(&stream);
    println!("Sending code 100");
    out_buffer.write_all("100\n".as_bytes());
    out_buffer.flush();
   
    /* wait until the server responds with "200" (standing by for IO), then send
     * the user data */
    let in_buffer = BufReader::new(&stream);

    println!("Waiting for response...");
    for line in in_buffer.lines() {
        let current_line = line.unwrap();
        let status_code = &current_line[0..3];

        println!("Server status code {}", status_code);

        match status_code {
            "201" => {
                println!("Expecting IO...");
                intial_connection(&stream);
            }
            "200" => {
                println!("Operation Successful!");
            }
            _ => {
                println!("Operation Failed!");
            }
            
        }
        
    }
}

/* this function returns dummy data for a user based in Aberystwyth on a 4G
 * mobile connection (haha). See the post.rs module for implementations and
 * the serialisation functions */
fn example_user_data() -> User
{
	let name: String = String::from("aberystwyth_seagull");
	let example_user = User{
		user_name: name,
		user_id: 0958467049586734098,
		latitude: 52.41612,
		longitude: -4.083798,
		range: 5,
		connection_type: 4
	};

	return example_user;
}
