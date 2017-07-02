/* set up our dependencies of which there are many */
extern crate byteorder;
extern crate ansi_term;
extern crate postgres;
extern crate rand;
//#[macro_use] extern crate text_io;

use std::io::*;
use std::net::{TcpListener, TcpStream, Shutdown};
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

    /* This is the main event loop driving the server */
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
            /* client terminating session */
            "101" => {
                stream::terminate(&stream);
                stream.shutdown(Shutdown::Both).expect("shutdown call failed");
            }
            /* client voting on a post */
            "102" => {
                client::vote(&stream, &dbase);
            }
            /* client adding a post */
            "103" => {
                client::add_post(&stream, &dbase);
            }
            /* client requesting post deletion */
            "104" => {
                user_data = client::del_post(&stream, &dbase, user_data);
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
    println!("Configuration file corrupted or missing, using defaults");
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
