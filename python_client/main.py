#Basic MooseCast Client for testing the TCP server
#Version 0.1.0
#Author: Joe Thompson (aberystwyth_seagull)

import socket
import post

TCP_IP = 'localhost'
TCP_PORT = 1337
BUFFER_SIZE = 20  # Normally 1024, but we want fast response

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind((TCP_IP, TCP_PORT))
s.listen(1)

conn, addr = s.accept()
print 'Connection address:', addr
while 1:
    data = conn.recv(BUFFER_SIZE)
    if not data: break
    print "received data:", data
    conn.send(data)  # echo
conn.close()