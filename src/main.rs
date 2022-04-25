use std::time::{SystemTime};
use rand::Rng;
// use std::io;

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



// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
// graveyard below %%%%%%%%%%%%%%%%%%%%%%%%%%
    /* 




// section 5.2 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@

    let rect1 = Rectangle {
        width: 30, 
        height: 50, 
    };
    println!("Area of given rect is {}", 
                area(&rect1));

struct Rectangle {
    width: u32, 
    height: u32, 
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// section 5.1 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@

    let mut user1 = User {
        email: String::from("me@email.gov"), 
        username: String::from("howdy"), 
        active: true, 
        sign_in_count: 1, 
    };
    println!("user1 email is {}", user1.email);
    user1.email = String::from("you@email.gov");
    println!("email updated to {}", user1.email);

    let user2 = build_user(String::from("u2@me.org"), 
                            String::from("Iluvyou"));
    println!("user number 2 email is {}", user2.email);

    let user3 = User {
        email: String::from("mr3@user.com"), 
        ..user1
    };
    println!("user number 3 email is {}", user3.email);
    //  println!("user number 1 email is {}", user1.username); based on this code, user1's username is overwritten when user 3 was created
    println!("user number 2 uname is {}", user2.username);

    let black = Color(0, 0, 0);
    let origin = Pointxy(0, 0); 
    println!("color black second digit is {}", black.1);
    println!("point origin x is {}", origin.0);

struct User {
    active: bool, 
    username: String,
    email: String,
    sign_in_count: u64,  
}

fn build_user(email: String, username: String) -> User {
    User {
        email, 
        username, 
        active: true, 
        sign_in_count: 1, 
    }
}

struct Color(i32, i32, i32);
struct Pointxy(i32, i32);


// section 4.3 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@

    let x = b' ';
    println!("byte literal for a space is {}", x);

    let s = String::from("hello world");
    println!("{}", s);
    let slice = &s[0..5];
    println!("{}", slice);
    let slice = &s[..5];
    println!("{}", slice);
    let slice = &s[6..];
    println!("{}", slice);
    let lenn = s.len();
    let slice = &s[6..lenn];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);

    let stringy = "hello wordle";
    let printy = first_word(&stringy[..]);
    println!("{} but first, {}", stringy, printy);

    let a = [1, 2, 3, 4, 5];
    let crops = &a[1..2];

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


// section 4.2 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// section 4.1 @@@@@@@@@@@@@@@@@@@@@@@@@@@@@

    {
    let mut s = String::from("hello");
    s.push_str(", world"); 
    println!("{}", s);
    }

    let x = String::from("hello"); 
    let y = x.clone();
    println!("x = {}, y = {}", x, y); 

    
// section 3.5 @@@@@@@@@@@@@@@@@@@@@@@@@@

    let numero = 2;

    if numero % 2 == 0 {
        println!("Divisible by 2");
    } else if numero % 3 == 0 {
        println!("Divisible by 3");
    } else {
        println!("Condition false");
    }

    let condy = false;
    let numero = if condy { 99 } else { 98 };
    println!("{}", numero);

    /*loop {
        println!("Im LOOPING bruv! CTRL+C to KILL");
    }*/

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 100 {
            break counter;
        }
    };
    println!("Counter Loop Completed @ {}", result);

    let mut numero = 3; 
    while numero != 0 {
        println!("{}", numero);
        numero -= 1; 
    }
    println!("BANG");

    let a = [10, 20, 30, 40 , 50];
    let mut index = 0;
    while index < 5 {
        println!("the array val @ index {} is {}", index, a[index]);
        index += 1;
    } // fragile and should be converted to for, like below

    let a = [10, 20, 30, 40 , 50, 60 , 70, 1];
    for element in a {
        println!("the array val is {}", element)
    }

    for nummm in (1..6).rev() {
        println!("{}", nummm);
    }
    println!("BANGG");

    // fibonnaci function test
    fn fib(n: u8) -> u64 {
        let mut prev: u64 = 0;
        let mut curr: u64 = 1;
        for _ in 1..n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        curr
    }
    println!("{}", fib(10));


// section 3.3 @@@@@@@@@@@@@@@@@@@@@

    learning_fun(3, 'b');

    let z = learning_fun_more(6);
    println!("Val is {}", z);

fn learning_fun(x: u8, unit_label: char) {
    println!("Look at me learn functions!");

    let y: u8 = x + 1;

    println!("Value of x is {}{}", y, unit_label);
}

fn learning_fun_more(x: u8) -> u8 {
    x + 1
}


// section 3.2 @@@@@@@@@@@@@@@@@@@@@
    
    let barf: f64 = 8.942 + 3.0;
    println!("Value = {}", barf);

    let barf: i8 = 13;
    println!("Value = {}", barf);

    let barf: bool = true;
    println!("Value = {}", barf);

    let barf = 'b';
    println!("Value = {}", barf);

    let barf = '😻';
    println!("Value = {}", barf);

    let barf: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = barf;
    println!("Value = {}", y);
    println!("Value = {}", barf.0);

    let barf = [1, 2, 3, 4, 5];
    println!("Value = {}", barf[1]);
    let barf: [i32; 5] = [0; 5];
    println!("Value = {}", barf[1]);
    let barf: [bool; 5] = [false; 5];
    println!("Value = {}", barf[1]);
    let months = ["January", "February", "March", 
                    "April", "May", "June", "July",
                    "August", "September", "October", 
                    "November", "December"];
    println!("Value = {}", months[0]);

    
// section 3.1 @@@@@@@@@@@@@@@@@@@@@
    let x = 5;
    println!("The val of x is {}", x);
    let x = x + 1; 
    println!("The val of x is {}", x);

    {
        let x = x * 2;
        println!("Inner scope x value is {}", x)
    }

    println!("The val of x is {}", x);

    let spaces_str = "       ";
    let spaces_num = spaces_str.len();
    println!("Number of spaces is {}", spaces_num);

    const FART: i32 = 7 + 8 - 10 * 90;
    println!("The constant is value {}", FART);    
    
*/