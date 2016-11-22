/* functions for encoding and decoding posts */
use std::io::Cursor;
use byteorder::{BigEndian, ByteOrder};
use std::str;

/*
 * this struct provides a template for posts as they exist within the server,
 * it is an easy intermediate form between the MYSQL database and the machine-
 * readable Anonymoose transmission format
 */
pub struct Post
{
 	pub post_id: i64, /* unique post ID */
 	pub timestamp: i32, /* UNIX timestamp of the post date */
 	pub latitude: f32, /* latitude in decimal degrees */
 	pub longitude: f32, /* longitude in decimal degrees */
 	pub upvotes: i16, /* number of upvotes */
 	pub downvotes: i16, /* number of downvotes */
 	pub flags: [bool; 12], /* store various flags */
 	pub replies: Vec<i64>, /* a vector containing the post IDs of replies */
 	pub text: Vec<u8>, /* UTF8 encoded post text as a byte array */
	pub binary: Vec<u8>, /* raw binary data for images/video ect. */
}

/* encodes post structs into the Anonymoose transmission format */
pub fn post_encode(target: Post) -> [u8; 1024]
{
	let mut post_buffer = [0; 1024];

	/* use ByteOrder to turn the data into a big-endian byte array */
	//BigEndian::write_i64(&mut post_buffer, "POST");
	BigEndian::write_i64(&mut post_buffer, target.post_id);
	BigEndian::write_i32(&mut post_buffer, target.timestamp);
	BigEndian::write_f32(&mut post_buffer, target.latitude);
	BigEndian::write_f32(&mut post_buffer, target.longitude);
	BigEndian::write_i16(&mut post_buffer, target.upvotes);
	BigEndian::write_i16(&mut post_buffer, target.downvotes);

	return post_buffer;
}

pub fn post_decode(buffer: [u8; 1024]) -> Post
{
	
}










