use hyper::Client;
use hyper::header::Connection;

use xml::reader::EventReader;
use xml::reader::events::*;

use models::Post;

// fn indent(size: usize) -> String {
//     const INDENT: &'static str = "    ";
//     (0..size).map(|_| INDENT)
//         .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
// }

pub fn download_and_parse_rss(url: &str) -> Vec<Post> {
    let mut client = Client::new();

    let res = client.get(url)
        .header(Connection::close())
        .send().unwrap();

    let mut parser = EventReader::new(res);

    let mut next_post:Option<Post> = None;
    let mut current_tag:Option<String> = None;

    let mut posts:Vec<Post> = vec![];

    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, .. } => {
                if name.local_name == "item" {
                    next_post = Some(Post{
                        author: None,
                        title: None,
                        link: None,
                        description: None,
                        guid: None
                    });
                } else {
                    current_tag = Some(name.local_name);
                }
            },
            XmlEvent::EndElement { name } => {
                if name.local_name == "item" {
                    posts.push(next_post.take().unwrap());
                } else {
                    current_tag = None;
                }
            },
            XmlEvent::Characters(characters) => {
                if let (Some(tag), Some(post)) =
                        (current_tag.as_ref(), next_post.as_mut()) {
                    match tag.as_ref() {
                        "author" => {
                            post.author = Some(characters);
                        },
                        "title" => {
                            post.title = Some(characters);
                        },
                        "link" => {
                            post.link = Some(characters);
                        },
                        "description" => {
                            post.description = Some(characters);
                        },
                        "guid" => {
                            post.guid = characters.parse().ok();
                        }
                        _ => {}
                    }
                }
            },
            _ => {},
        }
    }

    posts
}
