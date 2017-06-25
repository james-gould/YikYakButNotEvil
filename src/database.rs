/* provide the connection to the postgres database and utilities */
use postgres::{Connection, TlsMode};
use post::Post;
use post::User;

/* this function builds a new set of tables from scratch, for initialising new
 * instances and starting from scratch when shit hits the fan.
 */
pub fn init_tables(dbase: &Connection)
{
	/* this perfectly maps onto our post struct, how convenient is that? */
	dbase.execute("CREATE TABLE posts (
                    post_id         BIGINT PRIMARY KEY NOT NULL,
                    timestamp       INT NOT NULL,
                    latitude        REAL NOT NULL,
                    longitude       REAL NOT NULL,
                    upvotes         SMALLINT NOT NULL,
                    downvotes       SMALLINT NOT NULL,
                    text            VARCHAR,
                    parent_id       BIGINT,
                    user_id         BIGINT

                  )", &[]).unwrap();
	
	/* same for the user struct except we don't need to store their location or 
	 * the range, we're not trying to build the marauder's map.
	 */
	dbase.execute("CREATE TABLE users (
                    user_id BIGINT PRIMARY KEY NOT NULL,
                    user_name VARCHAR,
                    karma INT NOT NULL
                  )", &[]).unwrap();

}

/* this adds a post into the database */
pub fn add_post(dbase: &Connection, target: Post)
{
	/* the execute function requires references as a parameter so we'll make our
		zero here */
	let zero: i16 = 0;
	dbase.execute("INSERT INTO posts (post_id, timestamp, latitude, longitude,
						upvotes, downvotes, text, parent_id, user_id)
						VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                 &[&target.post_id, &target.timestamp, &target.latitude,
                 	&target.longitude, &zero, &zero, &target.text,
                 	&target.parent_id, &target.user_id]).unwrap();
}

/* this retrieves posts from the database */
pub fn get_posts(dbase: &Connection, user: &User) -> Vec<Post>
{
	/* this is fairly straightforward, get range of latitudes within 5mi,
		this is a simple constant as distances between latitudes are constant */
	let max_latitude: f32 = user.latitude + 0.073;
	let min_latitude: f32 = user.latitude - 0.073;

	/* now we need longitudes, this requires some trigonometry as they vary */
	let miles_per_degree: f32 = user.latitude.cos().to_degrees() * 68.703
		as f32;
	let degrees_per_five_miles: f32 = (1f32 / miles_per_degree) * 5f32;

	let max_longitude = user.longitude + degrees_per_five_miles;
	let min_longitude = user.longitude - degrees_per_five_miles;

	/* this vector will store the returned posts */
	let mut posts_buffer: Vec<Post> = Vec::new();

	/* query the database to find all posts within the range */
	for row in &dbase.query("SELECT * FROM posts WHERE latitude BETWEEN $1 AND
								$2 AND longitude BETWEEN $3 AND $4",
							&[&max_latitude, &min_latitude, &max_longitude,
								&min_longitude]).unwrap() { 
	    let current_post = Post {
	    	post_id: row.get(0),
	    	timestamp: row.get(1),
	    	latitude: row.get(2),
		 	longitude: row.get(3),
		 	upvotes: row.get(4),
		 	downvotes: row.get(5),
		 	text: row.get(6),
		 	parent_id: row.get(7),
		 	user_id: row.get(8), 	

	    };
	    
	    /* put the found posts into the buffer */
	    posts_buffer.push(current_post);
	}
	return posts_buffer;
}

pub fn vote(dbase: &Connection, mode: i8, post_id: i64)
{
	let trn = dbase.transaction().unwrap();
	if mode == 0 {
		trn.execute("UPDATE posts
						SET upvotes = upvotes + 1
						WHERE post_id = $1", &[&post_id]).unwrap();
	}
	else {
		trn.execute("UPDATE posts
						SET downvotes = downvotes + 1
						WHERE post_id = $1", &[&post_id]).unwrap();
	}
	trn.commit().unwrap();
}











