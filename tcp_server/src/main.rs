/* set up our dependencies of which there are many */
extern crate byteorder;
extern crate ansi_term;
extern crate postgres;
extern crate rand;
#[macro_use] extern crate text_io;

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
 * this function drives client-server interaction, responding to status codes
 */
fn handle_client(stream: TcpStream)
{
    /* connect to the Postgres database */
    println!("Connecting to the post database...");
    let dbase = Connection::connect("postgres://josephthompson:tiger@localhost",
        TlsMode::None).unwrap();
    
    /* this holds the user information */
    let user_data: post::User;
    
    /*read the TCP stream into a buffer*/
    let buffer = BufReader::new(&stream);

    /*get the first four chars as a string slice to determine the next action*/
    for line in buffer.lines() {
        let current_line = line.unwrap();
        println!("Client status code {}", current_line);

        let status_code = &current_line[0..3];

        match status_code {
            "100" => {
                /* get information about the client */
                let user_data = stream::initial_connection(&stream);

                /* print username and id */
                println!("{} connected with ID {}", user_data.user_name, user_data.user_id);

                /* retrieve nearby posts from the database */
                let posts_buffer: Vec<post::Post> =
                    database::get_posts(&dbase, user_data);

                /* serialise post into the AM format for transmission */
                let mut out_buffer: Vec<u8> = Vec::new();
                for p in posts_buffer {
                   let raw_data = post::post_encode(p);
                   /* push each byte of the newly encoded data to the buffer */
                   for byte in raw_data {
                        out_buffer.push(byte);
                   }
                }
                stream::send_to_client(&stream, out_buffer);
            }
            "102" => {
                /* accept the raw AM format post data */
                let raw_post: Vec<u8> = stream::recieve_from_client(&stream);

                /* convert this data into a Post struct */
                let post: post::Post = post::post_decode(raw_post);

                /* add this post to the database */
                database::add_post(&dbase, post);

                stream::success(&stream);
            }
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

    println!("TCP Server version ALPHA 0.0.2");
    println!("Copyright (c) The Anonymoose Team, all rights reserved.");
    println!("Starting...\n");

    /* load configuration */
    println!("Loading configuration...");
    println!("Done!");    

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
