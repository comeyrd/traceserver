# Traceserver 

This project is a Team project with [katheleligaf](https://github.com/katheleligaf), [n4n5](https://github.com/Its-Just-Nans), [SachaDico](https://github.com/SachaDico)

Traceserver is a websocket server, and for every websocket message it receives, it sends back a json like this : 

{
    direction:"upload" or "download",
    rate:int16,
    text:char,
    roaming:bool
}

The value inside the Json is randomly generated.
This project is linked to [Tracefront](https://github.com/comeyrd/tracefront), it is an application that displays theses traces.

You can query an instance of this project at this address : wss://ws.ceyraud.com
