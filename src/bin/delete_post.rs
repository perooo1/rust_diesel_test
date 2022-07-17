extern crate diesel;
extern crate diesel_demo;

use diesel::connection;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;
use std::fmt::format;

fn main() {
    use diesel_demo::schema::posts::dsl::*;

    let target = args().nth(1).expect("Argument not valid");
    let pattern = format!("%{}%",target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting post");

    println!("Deleted {} posts!", num_deleted);


}
