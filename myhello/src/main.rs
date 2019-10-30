extern crate tokio;

use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::net::TcpListener;

fn client() {
// Parse the address of whatever server we're talking to
    let addr = "127.0.0.1:6142".parse().unwrap();
    let client = TcpStream::connect(&addr).and_then(|stream| {
        println!("created stream");
        // Process stream here.
        //for x in 0..10 {
                io::write_all(stream, b"hello world").then(|result| {
                    println!("wrote to stream; success={:?}", result.is_ok());
                    Ok(())
                })
        //}

    }).map_err(|err| {
        // All tasks must have an `Error` type of `()`. This forces error
        // handling and helps avoid silencing failures.
        // In our example, we are only going to log the error to STDOUT.
        println!("connection error = {:?}", err);
    });

    println!("About to create the stream and write to it...");
    tokio::run(client);
    println!("Stream has been created and written to.");
}

fn server() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` combinator method
    let server = listener.incoming().for_each(|socket| {
      // split the socket stream into readable and writable parts
      let (reader, writer) = socket.split();
      // copy bytes from the reader into the writer
      let amount = io::copy(reader, writer);
      //println!("received {}",socket.read_to_end());

      let msg = amount.then(|result| {
        match result {
          Ok((amount, _, _)) => println!("wrote {} bytes", amount),
          Err(e)             => println!("error---: {}", e),
        }

        Ok(())
      });

      // spawn the task that handles the client connection socket on to the
      // tokio runtime. This means each client connection will be handled
      // concurrently
      tokio::spawn(msg);
      Ok(())
    })
    .map_err(|err| {
        // Handle error by printing to STDOUT.
        println!("accept error = {:?}", err);
    });

    println!("server running on localhost:6142");

    // Start the server
    //
    // This does a few things:
    //
    // * Start the Tokio runtime
    // * Spawns the `server` task onto the runtime.
    // * Blocks the current thread until the runtime becomes idle, i.e. all
    //   spawned tasks have completed.
    tokio::run(server);
}

fn main() {
    client()
    //server()
}
