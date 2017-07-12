/* set up our dependencies of which there are many */
extern crate byteorder;
extern crate postgres;
extern crate rand;
extern crate json;

//use std::env;
use std::io::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use postgres::{Connection, TlsMode};
use std::thread;
use std::str;
use std::fs::File;

/* include the other parts of the server */
mod stream;
mod post;
mod database;
mod client;


/* this function gets the configuration for this instance */
fn get_config() -> json::JsonValue
{
    /* open the config file */
    let mut config_file = match File::open("config.json") {
        Err(why) => panic!("Config file load failed with error {}", why),
        Ok(config_file) => config_file,
    };

    /* read it to a string */
    let mut config_string: String = String::new();
    config_file.read_to_string(&mut config_string).unwrap();

    /* parse the Json */
    let config = json::parse(&config_string[..]).unwrap();

    return config;
}

/* 
 * this function drives client-server interaction, responding to status codes
 */
fn handle_client(stream: TcpStream, config: json::JsonValue)
{   
    /* set up database config */
    let mut config_string = String::from("postgres://");    
    config_string.push_str(
        config["postgres_config"]["username"].as_str().unwrap());
    
    config_string.push(':');
    
    config_string.push_str(
        config["postgres_config"]["password"].as_str().unwrap());
    
    config_string.push('@');
    
    config_string.push_str(
        config["postgres_config"]["host"].as_str().unwrap());

    /* connect to the Postgres database */
    println!("Connecting to the post database...");
    
    let dbase: Connection;

    match Connection::connect(&config_string[..],
        TlsMode::None) {
        Ok(n) => {
            println!("Connection okay!");
            dbase = n;
        }
        Err(e) => {
            println!("Error connecting to database, {}", e);
            stream::error(&stream, 304);
            panic!("Don't actually panic, sort this out later.");
        }
    }

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
                stream::success(&stream);
            }
            "107" => {
                client::get_replies(&stream, &dbase);
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
                                                 
                                                 
    println!("TCP Server version ALPHA 0.1.0");
    println!("Copyright (c) The MooseCast Team 2017, all rights reserved.");
    println!("Starting...\n");

    /* load configuration */
    println!("Loading configuration...");
    let config = get_config();
    println!("Success!");
    
    
    /* handle shell arguments
    for argument in env::args() {
        match argument {
            /* this is the configuration folder */
            "-c" => {
                config = config::read_config(argument.next());
            }
            _ => continue,
        }
    } */

    /* start listening for clients */
    let listener = TcpListener::bind("localhost:1337").unwrap();

    println!("TCP listener local information: {:?}\n", listener.local_addr());

    /* listen for TCP streams and stick each into its own thread */
    for stream in listener.incoming() {
        
        /* we want to throw our TCP connection around */
        let stream = stream.unwrap();

        /* since we're moving into a new thread, clone the config */
        let cloned_config = config.clone();
        
        thread::spawn(move || {
            println!("Incoming connection...");
            println!("Client information: {:?}", stream.peer_addr());
            handle_client(stream, cloned_config);
        });
    }
}
