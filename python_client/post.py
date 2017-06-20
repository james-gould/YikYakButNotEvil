# AM Protocol implementation and IO module
# Author: Joe Thompson (aberystwyth_seagull)

# approximation of the Post struct in the TCP server
class post:
	post_id = 0 # unique post ID
 	timestamp = 0 # UNIX timestamp of the post date
 	latitude = 0.0 # latitude in decimal degrees
 	longitude = 0.0 # longitude in decimal degrees
 	upvotes = 0 # number of upvotes
 	downvotes = 0 # number of downvotes
 	text = 'null' # post body
 	parent_id = 0 # parent ID, 0 if root post
 	user_id = 0 # unique user ID	