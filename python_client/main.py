#Basic MooseCast Client for testing the TCP server
#Version 0.1.0
#Author: Joe Thompson (aberystwyth_seagull)

import socket
import post

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server_address = ('localhost', 1337)
sock.connect(server_address)

sock.sendall("100\n")
