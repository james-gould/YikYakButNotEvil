/* crate root for the Anonymoose server, accepts TCP streams from clients and handles them */
extern crate byteorder;

use std::io::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::str;

/* include the other parts of the server */
mod stream;
mod post;
mod self_test;

/* command interpreter function */
fn command()
{
    let mut cmd_buffer = String::new();
    println!("Enter a command");
    loop {
        print!(">");
        std::io::stdin().read_to_string(&mut cmd_buffer);
        let cmd_slice = &cmd_buffer[0..4];

        match cmd_slice {
            "test" => println!("performing self-test"),
            _ => println!("Improper Command!"),
        }
    }
}


/* 
 * this function handles incoming TCP streams and calls the required function for their further
 * processing
 */
fn handle_client(mut stream: TcpStream)
{

    /*read the TCP stream into a buffer*/
    let buffer = BufReader::new(&mut stream);

    /*get the first four chars as a string slice to determine the next action*/
    for line in buffer.lines() {
        let current_line = line.unwrap();
        println!("Client status code {}", current_line);

        let status_code = &current_line[0..3];

        match status_code {
            "100" => println!("sending posts to client"),
            _ => println!("invalid status code!"),
        }
    }

    // Write our response
    //stream.write(response).unwrap();
}

fn main()
{
    println!("  ___                                                         ");
    println!(" / _ \\                                                        ");
    println!("/ /_\\ \\_ __   ___  _ __  _   _ _ __ ___   ___   ___  ___  ___ ");
    println!("|  _  | '_ \\ / _ \\| '_ \\| | | | '_ ` _ \\ / _ \\ / _ \\/ __|/ _ \\");
    println!("| | | | | | | (_) | | | | |_| | | | | | | (_) | (_) \\__ \\  __/");
    println!("\\_| |_/_| |_|\\___/|_| |_|\\__, |_| |_| |_|\\___/ \\___/|___/\\___|");
    println!("                          __/ |");
    println!("                         |___/");
    println!("");

    println!("Server version 0.0.1 PROTOTYPE");
    println!("Copyright (c) Anonymoose Industries Ltd, all rights reserved.");
    println!("Starting...\n");
    let listener = TcpListener::bind("localhost:1337").unwrap();

    println!("TCP listener local information: {:?}\n", listener.local_addr());

    /* start the command interpreter */
    thread::spawn(move || { command(); });

    /* listen for TCP streams and stick each into its own thread */
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            println!("Incoming connection...");
            println!("Client information: {:?}", stream.peer_addr());
            handle_client(stream);
        });
    }
}
