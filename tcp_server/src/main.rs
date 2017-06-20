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
        println!("Status Code: {}", status_code);

        match status_code {
            /* initial connection, request user data and send nearby posts */
            "100" => {
                /* get information about the client */
                let user_data = stream::initial_connection(&stream);

                /* print username and id */
                println!("{} connected with ID {}", user_data.user_name,
                    user_data.user_id);

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
            /* client adding a post */
            "102" => {
                /* accept the raw AM format post data */
                let raw_post: Vec<u8> = stream::recieve_from_client(&stream);

                /* convert this data into a Post struct */
                let post: post::Post = post::post_decode(raw_post);

                /* add this post to the database */
                database::add_post(&dbase, post);

                stream::success(&stream);
            }
            /* client voting on a post */
            "103" => {
                /* split the string up into the mode switch and post ID */
                let mode_string = &current_line[4..4];
                /* remember to strip off the terminator */
                let proper_length = current_line.len() - 1;
                let post_id_string = &current_line[5..proper_length];
                
                let mode = mode_string.parse::<i8>().unwrap();
                let post_id = post_id_string.parse::<i64>().unwrap();

                /* update the database */
                database::vote(&dbase, mode, post_id);

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
