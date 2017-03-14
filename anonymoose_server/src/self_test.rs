use post;
use post::Post;
use post::post_encode;
use post::post_decode;

use stream;
use stream::send_post_to_client;
use std::net::{TcpStream};

/* returns an example post struct for testing and sanity checks */
fn postgen() -> Post
{
	let bar = String::from("Hello, world!");
	
	let example_flags: [u8; 12] = [1, /* gold mode post */
									 1, /* mod post */
									 0, /* sticky post */
									 0, /* flagged for spam */
									 1, /* flagged for abuse */
									 1, /* not safe for work */
									 1, /* not in use */
									 0, /* not in use */
									 1, /* not in use */
									 0, /* not in use */
									 0, /* not in use */
									 1]; /* flagged for deletion */


	let test_post = post::Post {post_id: 184467440737095, timestamp: 1477067677, 
		latitude: 0.564, longitude: 60.435, upvotes: 54, downvotes: 12,
		flags: example_flags, text: bar,
		parent_id: 1337, user_id: 1337};

	return test_post;
}

pub fn test(mut stream: &TcpStream)
{
	println!("Testing post transmission...");
	let test_post = postgen();
	let payload = post_encode(test_post);
	send_post_to_client(stream, payload);

	println!("Testing post decoding...");
	let test_post2 = postgen();
	let payload2: Vec<u8> = post_encode(test_post2);
	let decoded_post = post_decode(payload2);
	
	println!("\nStarted with:");
	post_dump(postgen());
	println!("\nFinished with:");
	post_dump(decoded_post);

}

/* dump the contents of a post struct to the stdin */
pub fn post_dump(target: Post) {
	println!("Post ID is {}", target.post_id);
	println!("Timestamp is {}", target.timestamp);
	println!("Latitude is {}", target.latitude);
	println!("Longitude is {}", target.longitude);
	println!("Upvote count is {}", target.upvotes);
	println!("Downvote count is {}", target.downvotes);
	println!("Body text is {}", target.text);
	println!("Parent ID is {}", target.parent_id);
	println!("User ID is {}", target.user_id);
}








