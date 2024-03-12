# Traceserver 

Traceserver is a websocket server, and for every websocket message it receives, it sends back a json like this : 

{
    direction:"upload" or "download",
    rate:int16,
    text:char,
    roaming:bool
}

The value inside the Json is randomly generated.
This project is linked to [Tracefront](https://github.com/comeyrd/tracefront), it is an application that displays theses traces.