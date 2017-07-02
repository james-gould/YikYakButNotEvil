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
		    let mut z: Option<post::User> = Some(u);
			return z;
		}
	}
}

pub fn get_user_data(stream: &TcpStream) -> post::User
{
	/* get information about the client */
    let user_data = stream::initial_connection(&stream);

    /* send 202 to tell the client this was successful */
    stream::ready(&stream);

    /* return the user data */
    return user_data;
}

pub fn vote(stream: &TcpStream, dbase: &Connection)
{
	/* tell the client we're ready for IO */
	stream::ready(&stream);

	/* retrieve the response */
    let mut raw_data: Vec<u8> = stream::recieve_from_client(&stream);

    /* the first byte tells us the direction, the rest gives us the ID */
    let post_id = post::deserialise_post_id(raw_data.split_off(1));
    let vote_direction: u8 = raw_data.pop().unwrap();

    /* update the database */
    database::vote(&dbase, vote_direction as i8, post_id);

}

pub fn add_post(stream: &TcpStream, dbase: &Connection)
{
	/* tell the client we're ready for IO */
	stream::ready(&stream);

	/* accept the raw AM format post data */
    let raw_post: Vec<u8> = stream::recieve_from_client(&stream);

    println!("got {} bytes", raw_post.len());

    /* convert this data into a Post struct */
    let post: post::Post = post::post_decode(raw_post);

    /* add this post to the database */
    database::add_post(&dbase, post);

    stream::success(&stream);
}

pub fn del_post(stream: &TcpStream, dbase: &Connection,
	user_data: Option<post::User>) -> Option<post::User>
{
	/* tell the client we're ready for IO */
	stream::ready(&stream);

	/* receive the post ID of the target and deserialise it */
	let raw_data = stream::recieve_from_client(&stream);
	let post_id = post::deserialise_post_id(raw_data);

	/* get a representation of the post in question so we can analyse */
	let p = database::get_post(dbase, post_id);

	/* make sure the user_id of the post matches the current user */
	match user_data {
		None => panic!("User Data Missing or Corrupted!"),
		Some(u) => {
			if u.user_id == p.user_id {
				database::delete_post(dbase, post_id);
				stream::success(&stream);

				/* pass the user_data struct back to the calling function */
		    	let mut z: Option<post::User> = Some(u);
				return z;
			}
			else {
				panic!("Attempted to delete another user\'s post!");
			}
		}
	}
	
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

