/* functions for interacting directly with the client */
use std::thread;
use std::net::{Shutdown, TcpStream};
use postgres::{Connection, TlsMode};

use post;
use stream;
use database;

pub fn print_user_data(user_data: Option<post::User>) -> Option<post::User>
{
	/* check the user data exists */
	match user_data {
		None => panic!("User Data Missing or Corrupted!"),
		Some(u) => {
		    
		    println!("User {} connected with ID {}", u.user_name, u.user_id);

		    /* pass the user_data struct back to the calling function */
		    let mut z: Option<post::User> = None;
		    z = Some(u);
			return z;
		}
	}
}

pub fn get_user_data(stream: &TcpStream) -> post::User
{
	/* get information about the client */
    let user_data = stream::initial_connection(&stream);

    /* send 202 to tell the client we're ready to send the nearby posts */
    stream::send_to_client(&stream, String::from("202\n").into_bytes());

    /* return the user data */
    return user_data;
}

pub fn add_post(stream: &TcpStream, dbase: &Connection)
{
	/* accept the raw AM format post data */
    let raw_post: Vec<u8> = stream::recieve_from_client(&stream);

    /* convert this data into a Post struct */
    let post: post::Post = post::post_decode(raw_post);

    /* add this post to the database */
    database::add_post(&dbase, post);

    stream::success(&stream);
}

/* retrieve nearby posts from the database */
pub fn req_posts(stream: &TcpStream, dbase: &Connection,
	user_data: Option<post::User>) -> Option<post::User>
{
	/* check the user data exists */
	match user_data {
		None => panic!("User Data Missing or Corrupted!"),
		Some(u) => {
			/* access the database */
			let posts_buffer: Vec<post::Post> =
		    	database::get_posts(&dbase, &u);

		    

		    /* serialise post into the AM format for transmission */
		    let mut out_buffer: Vec<u8> = Vec::new();
		    
		    for p in posts_buffer {
		        let raw_data = post::post_encode(p);
		        /* push each byte of the newly encoded data to the buffer */
			    for byte in raw_data {
			        out_buffer.push(byte);
			    }
		    	out_buffer.push(0x3);
		    }
		    stream::send_to_client(&stream, out_buffer);
		    
		    /* pass the user_data struct back to the calling function */
		    let mut z: Option<post::User> = None;
		    z = Some(u);
			return z;
		}
	}
}

