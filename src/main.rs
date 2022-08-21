use std::{time::{SystemTime}};
use rand::Rng;
// use std::io::{self, Write};
//use crate::playpen::fibonnaci;

// bring in sibling .rs files
//mod playpen;

fn main() {

    // const SESSION_START_DTTIME = SystemTime::now(); 
    println!("\nwe talking bout practice!\n");
    // println!("Session start = {}", SESSION_START_DTTIME);
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago", n.as_secs()),
        Err(_) => panic!("time is broken"),
    }

    let session_random_num: u32 = rand::thread_rng().gen_range(100000..1000000);
    println!("here is a random number, {}", session_random_num);

    const WELCOME: &str = r#"       
    welcome to...
                   _   
    _ __ _   _ ___| |_
   | '__| | | / __| __|
   | |  | |_| \__ \ |_
   |_|   \__,_|___/\__|
   
    "#;
    println!("\n{}", WELCOME);

    /*let foob = fibonnaci(2);
    println!("{}", foob); */
    println!("-------------");

    // scratchwork goes here ...............



} // end of main function 

// scratchwork goes here ...............


