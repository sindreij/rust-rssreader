#![feature(result_expect)]

extern crate rssreader;
extern crate iron;
extern crate router;
extern crate serde;
extern crate mount;
extern crate staticfile;
extern crate time;
extern crate log;

use std::path::Path;

use rssreader::rss_parser;
use rssreader::db;
use rssreader::models::{Feed, Post};
use iron::prelude::*;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::status;
use router::Router;
use mount::Mount;
use serde::json;
use staticfile::Static;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

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

        //let posts:Vec<Post> = vec![];

        let posts =  rss_parser::download_and_parse_rss(&feed.url);

        db::add_posts(
            feed.id,
            posts.into_iter()
            .filter(|e|{
                !guids.contains(e.guid.as_ref().unwrap())
            })
        );
    }
}

fn get_feeds(_: &mut Request) -> IronResult<Response> {
    let feeds = db::get_feeds();
    let serialized = json::to_string(&feeds).unwrap();

    let content_type = Mime(TopLevel::Application, SubLevel::Json, vec![]);
    Ok(Response::with((status::Ok, serialized, content_type)))
}

fn get_posts(req: &mut Request) -> IronResult<Response> {
    println!("Get post");
    let feed_id: i32 = req.extensions.get::<Router>()
            .unwrap().find("feed_id").unwrap().parse().unwrap();
    let posts = db::get_posts(feed_id);
    let serialized = json::to_string(&posts).unwrap();

    let content_type = Mime(TopLevel::Application, SubLevel::Json, vec![]);
    Ok(Response::with((status::Ok, serialized, content_type)))
}

struct DebugMiddleware;

impl typemap::Key for DebugMiddleware { type Value = u64; }

impl BeforeMiddleware for DebugMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DebugMiddleware>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for DebugMiddleware {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<DebugMiddleware>().unwrap();
        println!("{} /{} [{} ms]", req.method, req.url.path.connect("/"), (delta as f64) / 1000000.0);
        Ok(res)
    }
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<DebugMiddleware>().unwrap();
        println!("ERR: {} {} {:?} [{} ms]", err, req.method, req.url.path.connect("/"), (delta as f64) / 1000000.0);
        Err(err)
    }
}

fn main() {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(SimpleLogger)
    }).unwrap();

    //update_feeds();

    let mut mount = Mount::new();
    let mut router = Router::new();
    router.get("/feed", get_feeds);
    router.get("/feed/:feed_id/posts", get_posts);
    router.get("/", Static::new(Path::new("static/dist/")));
    router.get("/*", Static::new(Path::new("static/dist/")));
    mount.mount("/", router);

    println!("On 3000");
    let mut chain = Chain::new(mount);
    chain.link_before(DebugMiddleware);
    chain.link_after(DebugMiddleware);
    let mut server = Iron::new(chain);
    server.http("127.0.0.1:3000").expect("Error serving");
}
