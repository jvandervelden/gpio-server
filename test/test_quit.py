import zmq

ctx = zmq.Context.instance()

socket = ctx.socket(zmq.REQ)

socket.connect("tcp://127.0.0.1:5555")

socket.send("00:00")

socket.close()
