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

fn initial_connection(stream: &TcpStream)
{
    /* serialise our user data and send it to the server */
    let vector = user_encode(example_user_data());
    send_to_server(&stream, vector);
}

fn main()
{
	println!("Anonymoose Dummy Client version 0.0.4");
    /* connect to the server - the default development server uses TCP port 1337
     * on localhost although this is configurable for obvious reasons */
    println!("Connecting to server...");
    let stream = TcpStream::connect("localhost:1337").unwrap();

    /* the protocol dictates that "100" is sent to the server to initialise the
     * connection */
    stream::send_100(&stream);
   
    let in_buffer = BufReader::new(&stream);

    /* so we can tell on the next iteration what we actually sent */
    let mut cmd_history: Vec<String> = Vec::new();
    /* we want 100 to be the first thing we send */
    cmd_history.push(String::from("100"));

    println!("Waiting for response...");
    for line in in_buffer.lines() {
        /* get the current server response and truncate to get the status code*/
        let current_line = line.unwrap();
        let status_code = &current_line[0..3];
        let prev_send_code = &cmd_history.pop().unwrap()[0..3];

        /* send the command to the server if we're not initialising the conn.*/
        if prev_send_code != "100" {
            stream::send_to_server(&stream, 
                String::from(prev_send_code).into_bytes());
        }

        println!("Server status code: {}", status_code);
        println!("Last command sent: {}", prev_send_code);

        match status_code {
            /* if we get a ready code, match the previous command and send
             * the appropriate data */
            "201" => {
                println!("Server Ready");
                match prev_send_code {
                    "100" => {
                        println!("Sending user data to the server");
                        initial_connection(&stream);
                    }
                    "102" => {
                        println!("Sending example post to server");
                        /* get an example post and turn it into bytes */
                        let buf: Vec<u8> = post_encode(example_post_data());
                        /* send her away! */
                        stream::send_to_server(&stream, buf);
                    }
                    _ => println!("Attempted Invalid Operation!"),
                }
            }
            "200" => {
                println!("Success!");
            }
            /* if it's anything else we've got ourselves an error */
            _ => {
                println!("Operation Failed With Error {}!", status_code);
            }
            
        }


        /* take input from the STDIN to dictate the next action */
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read from STDIN, got {}", n, input);
                /* save this command for the next iteration */
                cmd_history.push(input);
                }
            Err(error) => {
                println!("error: {}", error);
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






