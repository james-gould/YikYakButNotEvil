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
use std::time;
use std::str;
use rand::Rng;

/* include the other parts of the server */
mod stream;
mod post;
mod database;
mod client;
//mod validate;


/* 
 * this function drives client-server interaction, responding to status codes
 */
fn handle_client(stream: TcpStream)
{
    /* connect to the Postgres database */
    println!("Connecting to the post database...");
    let dbase = Connection::connect("postgres://josephthompson:tiger@localhost",
        TlsMode::None).unwrap();

    println!("done!");
    
    /* this holds the user information */
    let mut user_data: Option<post::User> = None;
    
    /*read the TCP stream into a buffer*/
    let buffer = BufReader::new(&stream);

    /*get the first four chars as a string slice to determine the next action*/
    for line in buffer.lines() {
        let current_line = line.unwrap();

        let status_code = &current_line[0..3];
        println!("Client Status Code: {}", status_code);

        match status_code {
            /* initial connection, request user data and send nearby posts */
            "100" => {
                user_data = Some(client::get_user_data(&stream));
                user_data = client::print_user_data(user_data);
            }
            /* client adding a post */
            "102" => {
                client::add_post(&stream, &dbase);
            }
            /* client voting on a post */
            "103" => {
                /* split the string up into the mode switch and post ID */
                //let mode_string = &current_line[4..5];
                
                /* remember to strip off the terminator */
                //let proper_length = current_line.len() - 1;
                //let post_id_string = &current_line[5..proper_length];

                //let mode = mode_string.parse::<i8>().unwrap();
                //let post_id = post_id_string.parse::<i64>().unwrap();

                /* update the database */
                //database::vote(&dbase, mode, 45634745746);
            }
            /* client requesting nearby posts */
            "106" => {
                user_data = client::req_posts(&stream, &dbase, user_data);
            }
            _ => println!("Client Sent Invalid Status Code..."),
        }
    }
}

fn main()
{
    println!("___  ___                     _____           _   ");
    println!("|  \\/  |                    /  __ \\         | |  ");
    println!("| .  . | ___   ___  ___  ___| /  \\/ __ _ ___| |_ ");
    println!("| |\\/| |/ _ \\ / _ \\/ __|/ _ \\ |    / _` / __| __|");
    println!("| |  | | (_) | (_) \\__ \\  __/ \\__/\\ (_| \\__ \\ |_ ");
    println!("\\_|  |_/\\___/ \\___/|___/\\___|\\____/\\__,_|___/\\__|");
                                                 
                                                 
    println!("TCP Server version ALPHA 0.0.3");
    println!("Copyright (c) The MooseCast Team 2016-17, all rights reserved.");
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
