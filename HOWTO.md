# How to Traceserver

To launch the webserver, you need to compile the server with cargo build --release.
You'll find the executable in ./target/release/tracefront

When the project is compiled, you can start a websocket server with :

tracefront IP PORT

If you want to have TLS capacity, you can run it behind a proxy that can take care of all the TLS and HTTPS problems.