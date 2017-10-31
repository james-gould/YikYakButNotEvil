/* functions for interacting directly with the client */
use std::net::{TcpStream, Shutdown};
use postgres::Connection;

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
		    let z: Option<post::User> = Some(u);
			return z;
		}
	}
}

pub fn get_user_data(stream: &TcpStream) -> post::User
{
	/* get information about the client */
    let user_data = stream::initial_connection(&stream);

    /* make sure we actually got something back */
    match user_data {
    	Some(u) => {
    		/* send 202 to tell the client this was successful */
    		stream::ready(&stream);
    		return u;
    	}
    	None => {
    		/* this would already have displayed an error on the console, so
    		 * silently send an error code to the client and kill the stream.
    		 * There's bugger all we can actually do if the user data is bad
    		 * except retrying from scratch as the whole event loop needs it */
    		stream::error(&stream, 302);
    		stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    		panic!("Don't actually panic, fix this when you're awake");
    	}
    }
}

pub fn vote(stream: &TcpStream, dbase: &Connection)
{
	/* tell the client we're ready for IO */
	stream::ready(&stream);

	let mut raw_data: Vec<u8>;

	/* retrieve the response */
    match stream::recieve_from_client(&stream) {
    	Some(n) => raw_data = n,
    	None => {
    		println!("Error, client sent no data!");
    		return ();
    	}
    }

    /* the first byte tells us the direction, the rest gives us the ID */
    let post_id = post::deserialise_post_id(raw_data.split_off(1));
    let vote_direction: u8;
    match raw_data.pop() {
    	Some(n) => vote_direction = n,
    	None => {
    		stream::error(&stream, 302);
    		return ();
    	}
    }

    /* update the database */
    database::vote(&dbase, vote_direction as i8, post_id);

}

pub fn add_post(stream: &TcpStream, dbase: &Connection)
{
	/* tell the client we're ready for IO */
	stream::ready(&stream);

	/* accept the raw AM format post data */
    let raw_post: Vec<u8>;

	/* retrieve the response */
    match stream::recieve_from_client(&stream) {
    	Some(n) => raw_post = n,
    	None => {
    		println!("Error, client sent no data!");
    		return ();
    	}
    }

    /* convert this data into a Post struct */
    let post: post::Post;
    match post::post_decode(raw_post) {
    	Ok(p) => post = p,
    	Err(e) => {
    		println!("Failed to decode post with error {}", e);
    		return ();
    	}
    }

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
	let raw_data: Vec<u8>;

	/* retrieve the response */
    match stream::recieve_from_client(&stream) {
    	Some(n) => raw_data = n,
    	None => {
    		println!("Error, client sent no data!");
    		/* pass the user_data struct back to the calling function 
				This is kind of a dangerous hack because we don't know
				we haven't been sent crappy user data, TODO fix this later!
    		*/
		    return Some(user_data.unwrap());
    	}
    }
	
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
		    	let z: Option<post::User> = Some(u);
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

		    /* serialise posts into the AM format for transmission */
		    let mut out_buffer: Vec<u8> = Vec::new();
		    
		    for p in posts_buffer {
		        let raw_data: Vec<u8> = post::post_encode(p);
		        /* push each byte of the newly encoded data to the buffer */
			    for byte in raw_data {
			        out_buffer.push(byte);
			        if byte == 0x3 {
			        	break;
			        }
			    }
		    }
		    
		    stream::send_to_client(&stream, out_buffer);

		    /* pass the user_data struct back to the calling function */
		    let z: Option<post::User> = Some(u);
			return z;
		}
	}
}

pub fn get_replies(stream: &TcpStream, dbase: &Connection)
{
	/* tell the client we're ready for IO */
	//stream::ready(&stream);

	let raw_data: Vec<u8>;

	/* retrieve the response */
    match stream::recieve_from_client_quiet(&stream) {
    	Some(n) => raw_data = n,
    	None => {
    		println!("Error, client sent no data!");
    		return ();
    	}
    }

    /* deserialise the post ID */
    let post_id = post::deserialise_post_id(raw_data);
    println!("Got post id {}", post_id.clone());

    /* get the replies from the database */
    let posts_buffer: Vec<post::Post> =
		    	database::get_replies(&dbase, post_id);

	/* serialise posts into the AM format for transmission */
	let mut out_buffer: Vec<u8> = Vec::new();
		    
	for p in posts_buffer {
		let raw_data: Vec<u8> = post::post_encode(p);
		/* push each byte of the newly encoded data to the buffer */
			for byte in raw_data {
			    out_buffer.push(byte);
			    if byte == 0x3 {
			        break;
			    }
			}
		}
		
	/* send them to the client */    
	stream::send_to_client(&stream, out_buffer);

}










