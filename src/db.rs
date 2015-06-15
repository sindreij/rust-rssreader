use std::collections::HashSet;

use postgres::{Connection, SslMode};
use postgres::error::ConnectError;

use models::{Feed, Post};

fn get_connection() ->Result<Connection, ConnectError> {
    let conn = try!(Connection::connect("postgres://sindre@%2Frun%2Fpostgresql/rssreader",
        &SslMode::None));
    Ok(conn)
}

pub fn create_tables() {
    let conn = get_connection().unwrap();

    let query = "
        CREATE TABLE feed (
            id SERIAL primary key,
            title VARCHAR(255) NOT NULL,
            url TEXT NOT NULL,
            last_retrieved TIMESTAMP,
            last_modified TIMESTAMP,
            etag TEXT
            )
    ";

    conn.execute(query, &[]).unwrap();

    let query = "
        CREATE TABLE post (
            id SERIAL primary key,
            feed int references feed(id) NOT NULL,
            author text,
            title text,
            link text,
            description text,
            guid text,
            )
    ";

    conn.execute(query, &[]).unwrap();
}

pub fn add_feed(feed: Feed) {
    let conn = get_connection().unwrap();

    let query = sql!("INSERT INTO feed (title, url) VALUES ($1, $2)");
    conn.execute(query, &[&feed.title, &feed.url]).unwrap();
}

pub fn get_feeds() -> Vec<Feed> {
    let conn = get_connection().unwrap();

    let query = sql!("SELECT id, title, url FROM feed");
    let stmt = conn.prepare(query).unwrap();
    stmt.query(&[]).unwrap().iter().map(|row| {
        Feed {
            id: row.get(0),
            title: row.get(1),
            url: row.get(2),
        }
    }).collect()
}

pub fn add_posts<I>(feed_id: i32, posts: I)
    where I: IntoIterator<Item=Post> {

    let conn = get_connection().unwrap();
    let trans = conn.transaction().unwrap();

    let query = sql!("INSERT INTO post (feed, author, title, link, description, guid)
        VALUES ($1, $2, $3, $4, $5, $6)");
    let stmt = trans.prepare_cached(query).unwrap();
    for post in posts {
        stmt.execute(&[
            &feed_id,
            &post.author,
            &post.title,
            &post.link,
            &post.description,
            &post.guid]).unwrap();
        println!("{:?}", post.title);
    }

    trans.commit().unwrap();
}

pub fn get_posts(feed_id: i32) -> Vec<Post> {
    let conn = get_connection().unwrap();

    let query = sql!("SELECT author, title, link, description, guid FROM post where feed=$1");
    let stmt = conn.prepare(query).unwrap();
    stmt.query(&[&feed_id]).unwrap().iter().map(|row| {
        Post {
            author: row.get(0),
            title: row.get(1),
            link: row.get(2),
            description: row.get(3),
            guid: row.get(4),
        }
    }).collect()
}

pub fn get_post_guid_map(feed_id: i32) -> HashSet<String> {
    let conn = get_connection().unwrap();

    let query = sql!("SELECT guid FROM post where feed=$1");
    let stmt = conn.prepare(query).unwrap();
    stmt.query(&[&feed_id]).unwrap().iter().map(|row| {
        row.get(0)
    }).collect()
}
