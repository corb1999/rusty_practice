use std::{time::{SystemTime, Duration}};
use rand::Rng;
use std::thread::sleep; 
// use std::io::{self, Write};
use crate::playpen::fibonnaci;

// bring in sibling .rs files
mod playpen;

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
   
                          \\/
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒
    "#;
    println!("\n{}\n", WELCOME);

    let foob = fibonnaci(2);
    println!("{}", foob); 
    println!("-------------");

    // scratchwork goes here ...............

    let store = Inventory {
      shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue], 
    };

    let user_pref1 = Some(ShirtColor::Red);
    
    let giveaway1 = store.giveaway(user_pref1); 

    println!("User with preference {:?} gets {:?}", user_pref1, giveaway1); 

    let user_pref2 = None; 
    let giveaway2 = store.giveaway(user_pref2); 
    println!("User with preference {:?} gets {:?}", user_pref2, giveaway2); 

    println!("-------------");

    let expensive_closure = |num: u32| -> u32 {
      println!("calculating slowly bruv...");
      std::thread::sleep(Duration::from_secs(2));
      num
    };


    let list = vec![1, 2, 3]; 
    println!("Before closure defined... {:?}", list); 

    let only_borrows = || println!("From closure: {:?}", list); 

    println!("Closure defined but not called... {:?}", list); 
    only_borrows(); 
    println!("Closure is now called so... {:?}", list);

    println!("-------------");
    
    let mut list2 = vec![1, 2, 3]; 
    println!("Before closure defined... {:?}", list2);

    let mut borrow_mutably = || list2.push(7); 

    borrow_mutably(); 
    println!("Closure is now called so... {:?}", list2);

    println!("-------------");

    let mut list3 = [
      Rectangle { width: 10, height: 1}, 
      Rectangle { width: 3, height: 5}, 
      Rectangle { width: 7, height: 12}, 
    ];

    list3.sort_by_key(|r| r.width); 
    println!("{:#?}", list3); 


} // end of main function 

// scratchwork goes here ...............

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red, 
    Blue, 
}

struct Inventory {
  shirts: Vec<ShirtColor>, 
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0; 
        let mut num_blue = 0;

        for color in &self.shirts {
          match color {
            ShirtColor::Red => num_red += 1, 
            ShirtColor::Blue => num_blue += 1, 
          }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
          ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
  width: u32, 
  height: u32, 
}
