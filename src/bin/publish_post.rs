extern crate diesel;
extern crate diesel_demo;

use diesel::QueryDsl;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::Post;
use std::env::args;

fn main(){
    use diesel_demo::schema::posts::dsl::{posts, pubblished};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid post id");

    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(pubblished.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post {} !", post.title);

}