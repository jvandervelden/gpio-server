import zmq
import sys

print("Sending Message: " + sys.argv[1])

ctx = zmq.Context.instance()

socket = ctx.socket(zmq.REQ)

socket.connect("tcp://127.0.0.1:5555")

socket.send_string(sys.argv[1])

retMessage = socket.recv()

print(retMessage)

socket.close()
