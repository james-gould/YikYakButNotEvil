/* functions for encoding, decoding and general post IO */
use byteorder::{BigEndian, ByteOrder};
use rand::*;


/*
 * this struct provides a template for posts as they exist within the server,
 * it is an easy intermediate form between the postgres database and the machine
 * -readable Anonymoose transmission format.
 */
pub struct Post
{
 	pub post_id: i64, /* unique post ID */
 	pub timestamp: i32, /* UNIX timestamp of the post date */
 	pub latitude: f32, /* latitude in decimal degrees */
 	pub longitude: f32, /* longitude in decimal degrees */
 	pub upvotes: i16, /* number of upvotes */
 	pub downvotes: i16, /* number of downvotes */
 	pub text: String, /* post body */
 	pub parent_id: i64, /* parent ID, 0 if root post */
 	pub user_id: i64, /* unique user ID */ 	
}

/* this struct describes a connected user */
pub struct User
{
	pub user_name: String, /* username - optional */
	pub user_id: i64, /* unique user ID */
	pub latitude: f32, /* latitude in decimal degrees */
	pub longitude: f32, /* longitude in decimal degrees */
	pub range: i16, /* selected range in miles */
	pub connection_type: u8 /* connection type (2G, 3G, 4G, wifi) */ 
}

/* encodes post structs into the Anonymoose transmission format */
pub fn post_encode(target: Post) -> Vec<u8>
{
	/* encode the text */
	let raw_text = target.text;
	let text_buffer = raw_text.as_bytes();

	/* encode the post header */
	let header_buffer = "POST".as_bytes();

	/* encode the post ID */
	let mut post_id_buffer = [0; 8];
	BigEndian::write_i64(&mut post_id_buffer, target.post_id);
	
	/* encode the timestamp */
	let mut timestamp_buffer = [0; 4];
	BigEndian::write_i32(&mut timestamp_buffer, target.timestamp);

	/* encode the latitude */
	let mut latitude_buffer = [0; 4];
	BigEndian::write_f32(&mut latitude_buffer, target.latitude);

	/* encode the longitude */
	let mut longitude_buffer = [0; 4];
	BigEndian::write_f32(&mut longitude_buffer, target.longitude);

	/* encode the upvotes */
	let mut upvotes_buffer = [0; 2];
	BigEndian::write_i16(&mut upvotes_buffer, target.upvotes); 
	/* encode the downvotes */
	let mut downvotes_buffer = [0; 2];
	BigEndian::write_i16(&mut downvotes_buffer, target.downvotes);

	/* encode the parent ID */
	let mut parent_id_buffer = [0; 8];
	BigEndian::write_i64(&mut parent_id_buffer, target.parent_id);

	/* encode the user ID */
	let mut user_id_buffer = [0; 8];
	BigEndian::write_i64(&mut user_id_buffer, target.user_id);

	/* now put all these byte arrays into a single vector for transmission */
	let mut post_buffer: Vec<u8> = Vec::new();

	/* end the post with ETX */
	let etx: u8 = 0x03;

	post_buffer.extend_from_slice(&header_buffer);
	post_buffer.extend_from_slice(&post_id_buffer);
	post_buffer.extend_from_slice(&timestamp_buffer);
	post_buffer.extend_from_slice(&latitude_buffer);
	post_buffer.extend_from_slice(&longitude_buffer);
	post_buffer.extend_from_slice(&upvotes_buffer);
	post_buffer.extend_from_slice(&downvotes_buffer);
	post_buffer.extend_from_slice(&parent_id_buffer);
	post_buffer.extend_from_slice(&user_id_buffer);
	post_buffer.extend_from_slice(&text_buffer);
	post_buffer.push(etx);
	
	return post_buffer;
}

/* splits up the incoming byte stream and returns a Post struct based on the
 * data it contains */
pub fn post_decode(mut target: Vec<u8>) -> Post
{
	println!("{}", target.len());

	/* decode the text */
	let text_buffer = target.split_off(44);
	let text = String::from_utf8(text_buffer).unwrap();

	/* decode the user ID */
	let user_id_vector = target.split_off(36);
	let user_id_buffer = &user_id_vector[..];
	let user_id = BigEndian::read_i64(user_id_buffer);

	/* decode parent ID */
	let parent_id_vector = target.split_off(28);
	let parent_id_buffer = &parent_id_vector[..];
	let parent_id = BigEndian::read_i64(parent_id_buffer);

	/* decode the longitude */
	let longitude_vector = target.split_off(20);
	let longitude_buffer = &longitude_vector[..];
	let longitude = BigEndian::read_f32(longitude_buffer);

	/* decode the latitude */
	let latitude_vector = target.split_off(16);
	let latitude_buffer = &latitude_vector[..];
	let latitude = BigEndian::read_f32(latitude_buffer);

	/* decode the timestamp */
	let timestamp_vector = target.split_off(12);
	let timestamp_buffer = &timestamp_vector[..];
	let timestamp = BigEndian::read_i32(timestamp_buffer);

	/* generate a unique post ID */
	let mut rng = thread_rng();
	let mut random: i64 = rng.gen();
	
	/* make sure the post ID is positive */
	if random < 0 {
		random = random * -1;
	}
	let post_id: i64 = random;

	let post_buffer = Post {post_id: post_id,
							timestamp: timestamp,
							latitude: latitude,
							longitude: longitude,
							upvotes: 0,
							downvotes: 0,
							text: text,
							parent_id: parent_id,
							user_id: user_id,
							}; 
	

	return post_buffer;

}

/* decode incoming user data and return a User struct */
pub fn user_decode(mut target: Vec<u8>) -> User
{
	if target.len() < 22 {
		panic!("Malformed user data!");
	}

	/* decode the username */
	let user_name_vector = target.split_off(23);
	let user_name = String::from_utf8(user_name_vector).unwrap();

	/* decode the connection type */
	let connection_type_vector = target.split_off(22);
	let connection_type = connection_type_vector[0];

	/* decode the range */
	let range_vector = target.split_off(20);
	let range_buffer = &range_vector[..];
	let range = BigEndian::read_i16(range_buffer);

	/* decode the longitude */
	let longitude_vector = target.split_off(16);
	let longitude_buffer = &longitude_vector[..];
	let longitude = BigEndian::read_f32(longitude_buffer);

	/* decode the latitude */
	let latitude_vector = target.split_off(12);
	let latitude_buffer = &latitude_vector[..];
	let latitude = BigEndian::read_f32(latitude_buffer);

	/* decode the user ID */
	let user_id_vector = target.split_off(4);
	let user_id_buffer = &user_id_vector[..];
	let user_id = BigEndian::read_i64(user_id_buffer);

	let user_buffer = User {user_name: user_name,
							user_id: user_id,
							latitude: latitude,
							longitude: longitude,
							range: range,
							connection_type: connection_type,
							};
	return user_buffer;
}

/* deserialises a post ID, used for voting, deleting, ect */
pub fn deserialise_post_id(target: Vec<u8>) -> i64
{
	let post_id_buffer = &target[..];
	let post_id = BigEndian::read_i64(post_id_buffer);
	return post_id;
}











