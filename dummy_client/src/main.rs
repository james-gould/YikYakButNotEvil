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

fn initial_connection(stream: &TcpStream) {
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
    stream::send_100(&stream);
   
    let in_buffer = BufReader::new(&stream);

    println!("Waiting for response...");
    for line in in_buffer.lines() {
        let current_line = line.unwrap();
        let status_code = &current_line[0..3];
        //let prev_code;

        println!("Server status code {}", status_code);

        match status_code {
            "202" => {
                initial_connection(&stream);
            }
            "201" => {
                println!("Expecting IO...");
                let posts_raw: Vec<u8> = stream::recieve_from_server(&stream);
            }
            "200" => {
                println!("Operation Successful!");
                send_to_server(&stream, post_encode(example_post_data()));
            }
            _ => {
                println!("Operation Failed!");
            }
            
        }

        /* take input from the STDIN to dictate the next action */
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read from STDIN", n);
                stream::send_to_server(&stream, input.into_bytes());
                continue;
            }
            Err(error) => {
                println!("error: {}", error);
                break;
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
	let example_user = User {
		user_name: name,
		user_id: 0958467049586734098,
		latitude: 52.41612,
		longitude: -4.083798,
		range: 5,
		connection_type: 4
	};

	return example_user;
}

fn example_post_data() -> Post
{
    let text: String = String::from("Hello World");
    let example_post = Post {
        post_id: 4570369845609,
        timestamp: 1498324432,
        latitude: 52.41612,
        longitude: -4.083700,
        upvotes: 78,
        downvotes: 14,
        text: text,
        parent_id: 0,
        user_id: 0958467049586734098
    };

    return example_post;
}






