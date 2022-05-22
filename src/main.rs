use std::{time::SystemTime};
use rand::Rng;
// use std::io::{self, Write};

fn main() {

    // const SESSION_START_DTTIME = SystemTime::now(); 
    println!("\nWe talking bout practice!\n");
    // println!("Session start = {}", SESSION_START_DTTIME);
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago", n.as_secs()),
        Err(_) => panic!("Time is broken"),
    }

    let session_random_num = rand::thread_rng().gen_range(100000..1000000);
    println!("Here is a random number, {}", session_random_num);

    const WELCOME: &str = r#"       
    welcome to...
                   _   
    _ __ _   _ ___| |_
   | '__| | | / __| __|
   | |  | |_| \__ \ |_
   |_|   \__,_|___/\__|"#;
    println!("\n{}\n", WELCOME);

    // scratchwork goes here ...............



} // end of main function 

// scratchwork goes here ...............


