/* set up our dependencies of which there are many */
extern crate byteorder;
extern crate ansi_term;
extern crate postgres;
extern crate rand;

use std::io::*;
use std::net::{TcpListener, TcpStream};
use postgres::{Connection, TlsMode};
use std::thread;
use std::str;
use rand::Rng;

/* include the other parts of the server */
mod stream;
mod post;
mod database;


/* 
 * this function handles incoming TCP streams and calls the required function for their further
 * processing
 */
fn handle_client(stream: TcpStream)
{
    /* this holds the user information */
    let mut user_data: post::User;
    
    /*read the TCP stream into a buffer*/
    let buffer = BufReader::new(&stream);

    /*get the first four chars as a string slice to determine the next action*/
    for line in buffer.lines() {
        let current_line = line.unwrap();
        println!("Client status code {}", current_line);

        let status_code = &current_line[0..3];

        match status_code {
            "100" => user_data = stream::initial_connection(&stream),
            //"100" => stream::ready(&stream),
            _ => println!("invalid status code!"),
        }
    }
}

fn main()
{
    println!("  ___                                                         ");
    println!(" / _ \\                                                        ");
    println!("/ /_\\ \\_ __   ___  _ __  _   _ _ __ ___   ___   ___  ___  ___ ");
    println!("|  _  | '_ \\ / _ \\| '_ \\| | | | '_ ` _ \\ / _ \\ / _ \\/ __|/ _\\");
    println!("| | | | | | | (_) | | | | |_| | | | | | | (_) | (_) \\__ \\  __/");
    println!("\\_| |_/_| |_|\\___/|_| |_|\\__, |_| |_| |_|\\___/ \\___/|___/\\___|");
    println!("                          __/ |");
    println!("                         |___/");
    println!("");  /* bloody hell */

    println!("TCP Server version 0.0.1");
    println!("Copyright (c) Anonymoose Industries Ltd, all rights reserved.");
    println!("Starting...\n");

    /* load configuration */
    println!("Loading configuration...");
    println!("Done!");

    /* connect to the postgres server */
    println!("Connecting to the post database");
    let dbase = Connection::connect("postgres://anonymoose:tiger@localhost",
        TlsMode::None).unwrap();
    

    let listener = TcpListener::bind("localhost:1337").unwrap();

    println!("TCP listener local information: {:?}\n", listener.local_addr());

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
