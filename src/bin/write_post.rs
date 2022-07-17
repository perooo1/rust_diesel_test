extern crate diesel_demo;
extern crate diesel;

use diesel::connection;

use self::diesel_demo::*;
use std::io::{stdin, Read};

fn main(){

    let connection = establish_connection();

    println!("WHAT WOULD YOU like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title ).unwrap();
    let title = &title[..(title.len()-1)];
    println!("\n Lets write {} (when finished press {})",title, EOF);

    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&connection, title, &body);
    println!("\n saved draft {} with id {}",title, post.id);

}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";