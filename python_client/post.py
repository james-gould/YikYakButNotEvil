# AM Protocol implementation and IO module
# Author: Joe Thompson (aberystwyth_seagull)

import struct
import time

# this handles post objects, in tcp_server they are represented by a struct
class Post:

	# constructor for the post class
 	def __init__(p, lat, lon, u, d, t, pid, uid):
 		post_id = p # unique post ID
	 	timestamp = time.time() # UNIX timestamp of the post date
	 	latitude = lat # latitude in decimal degrees
	 	longitude = lon # longitude in decimal degrees
	 	upvotes = u # number of upvotes
	 	downvotes = d # number of downvotes
	 	text = t # post body
	 	parent_id = pid # parent ID, 0 if root post
	 	user_id = uid # unique user ID

	# performs the same function as tcp_server's post_encode
	def encode(self):
		#output buffer for transmission
		out_buf = []

		#serialise the post fields
		struct.pack_into("c", out_buf, 0, "P")
		struct.pack_into("c", out_buf, 1, "O")
		struct.pack_into("c", out_buf, 2, "S")
		struct.pack_into("c", out_buf, 3, "T")
		struct.pack_into("q", out_buf, 4, post_id)
		struct.pack_into("l", out_buf, 12, timestamp)
		struct.pack_into("f", out_buf, 16, latitude)
		struct.pack_into("f", out_buf, 20, longitude)
		struct.pack_into("s", out_buf, 24, upvotes)
		struct.pack_into("s", out_buf, 26, downvotes)
		struct.pack_into("q", out_buf, 28, parent_id)
		struct.pack_into("q", out_buf, 36, user_id)
		for i in range(44..len(self.text)):
			struct.pack_into("c", out_buf, i, text[i])

		return out_buf

# this handles user objects, in tcp_server they are represented by a struct
class User:
	
	# constructor for the user class
	def __init__(u, uid, lat, lon, r, c):
		user_name = u # username (this is optional)
		user_id = uid # unique user ID
		latitude = lat # latitude in decimal degrees
		longitude = lon # longitude in decimal degrees
		rge = r # selected range in miles
		connection_type = c # connection type (2G, 3G, 4G, wifi)

	# encodes the user data for transmission
	def encode(self):
		#output buffer for transmission
		out_buf = []

		struct.pack_into("c", out_buf, 0, "I")
		struct.pack_into("c", out_buf, 1, "N")
		struct.pack_into("c", out_buf, 2, "I")
		struct.pack_into("c", out_buf, 3, "T")
		struct.pack_into("q", out_buf, 4, user_id)
		struct.pack_into("f", out_buf, 12, latitude)
		struct.pack_into("f", out_buf, 16, longitude)
		struct.pack_into("s", out_buf, 20, rge)
		struct.pack_into("c", out_buf, 22, connection_type)
		for i in range(23..len(self.user_name)):
			struct.pack_into("c", out_buf, i, user_name[i])

		return out_buf













