extern crate rssreader;
extern crate iron;
extern crate router;
extern crate serde;
extern crate mount;
extern crate staticfile;

use std::path::Path;

use rssreader::rss_parser;
use rssreader::db;
use rssreader::models::Feed;
use iron::prelude::*;
use iron::status;
use router::Router;
use mount::Mount;
use serde::json;
use staticfile::Static;

fn create_tables() {
    db::create_tables();
}

fn add_feed() {
    db::add_feed(Feed {
        id: -1,
        title: "The Daily WTF".to_string(),
        url: "http://syndication.thedailywtf.com/TheDailyWtf".to_string(),
    });
}

fn update_feeds() {
    for feed in db::get_feeds() {
        let guids = db::get_post_guid_map(feed.id);
        println!("{:?}, {:?}", feed.title, feed.url);

        let response =  rss_parser::download_rss(&feed.url);
        let posts = rss_parser::parse_rss(response);

        db::add_posts(
            feed.id,
            posts.into_iter()
            .filter(|e|{
                !guids.contains(e.guid.as_ref().unwrap())
            })
        );
    }
}

fn get_feeds(req: &mut Request) -> IronResult<Response> {
    let feeds = db::get_feeds();
    let serialized = json::to_string(&feeds).unwrap();

    Ok(Response::with((status::Ok, serialized)))
}

fn get_posts(req: &mut Request) -> IronResult<Response> {
    let feed_id: i32 = req.extensions.get::<Router>()
            .unwrap().find("feed_id").unwrap().parse().unwrap();
    let posts = db::get_posts(feed_id);
    let serialized = json::to_string(&posts).unwrap();

    Ok(Response::with((status::Ok, serialized)))
}

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("static/")));

    let router = {
        let mut router = Router::new();
        router.get("/feed/", get_feeds);
        router.get("/feed/:feed_id/posts/", get_posts);
        router
    };
    mount.mount("/api/", router);

    println!("On 3000");
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
