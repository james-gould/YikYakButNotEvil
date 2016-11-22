use post;
use post::Post;

/* returns an example post struct for testing and sanity checks */
fn postgen() -> Post
{
	let example_text = String::from("Hello, World!");
	let example_array = example_text.into_bytes();
	let example_data = vec![0xff, 0x00, 0x4b];
	let example_replies = vec![709551615, 457634568745, 3456345765473,
		36523464362];
	let example_flags: [bool; 12] = [true, /* gold mode post */
									 true, /* mod post */
									 false, /* sticky post */
									 false, /* flagged for spam */
									 true, /* flagged for abuse */
									 true, /* not safe for work */
									 true, /* not in use */
									 false, /* not in use */
									 true, /* not in use */
									 false, /* not in use */
									 false, /* not in use */
									 true]; /* flagged for deletion */


	let test_post = post::Post {post_id: 184467440737095, timestamp: 1477067677, 
		latitude: 0.564, longitude: 60.435, upvotes: 54, downvotes: 12,
		flags: example_flags, replies: example_replies, text: example_array,
		binary: example_data};

	return test_post;
}

/* attempts to encode the example post into the Anonymoose transmission format */
fn encode_test() -> [u8; 1024]
{
	let target = postgen();
	let result = post_encode(target);
	return result;
}

