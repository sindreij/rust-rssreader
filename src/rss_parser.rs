use hyper::Client;
use hyper::header::Connection;
use hyper::client::response::Response;

use xml::reader::EventReader;
use xml::reader::events::*;

use std::io::Read;

use itertools::Itertools;

use models::Post;

#[derive(Debug)]
enum TagOrPost {
    Tag{
        name: String,
        characters: String,
    },
    Post(Post),
}

#[derive(Debug)]
enum TagOrEvent {
    Tag{
        name: String,
        characters: String,
    },
    Event(XmlEvent),
}

// fn indent(size: usize) -> String {
//     const INDENT: &'static str = "    ";
//     (0..size).map(|_| INDENT)
//         .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
// }

pub fn download_rss(url: &str) -> Response {
    let mut client = Client::new();

    let res = client.get(url)
        .header(Connection::close())
        .send().unwrap();
    res
}

pub fn parse_rss<R: Read>(reader: R) -> Vec<Post> {
    let mut parser = EventReader::new(reader);

    //let mut items = Vec::new();

    let result = parser.events().filter(|e| {
            match e {
                &XmlEvent::StartElement { .. } => {
                    true
                    // name == "item"
                    // || name == "author"
                    // || name == "title"
                    // || name == "link"
                    // || name == "description"
                }
                &XmlEvent::Characters(..) => {
                    true
                }
                &XmlEvent::Error(..) => {
                    panic!("Lol, error in the document!");
                }
                _ => { false }
            }
        })
        .map(|x| TagOrEvent::Event(x))
        .coalesce(|x, y| {
            match (x, y) {
                (
                    TagOrEvent::Event(XmlEvent::StartElement{ name, .. }),
                    TagOrEvent::Event(XmlEvent::Characters(characters))
                ) => {
                    Ok(TagOrEvent::Tag{
                        name: name.local_name,
                        characters: characters
                    })
                },
                (
                    TagOrEvent::Tag{name, characters:current_chars},
                    TagOrEvent::Event(XmlEvent::Characters(characters))
                ) => {
                    Ok(TagOrEvent::Tag {
                        name: name,
                        characters: current_chars + &characters,
                    })
                },
                (a, b) => {
                    Err((a, b))
                }
            }
        }).filter_map(|x| {
            match x {
                TagOrEvent::Event(XmlEvent::StartElement{ name, ..}) => {
                    if name.local_name == "item" {
                        Some(TagOrPost::Post(Post{
                            author: None,
                            title: None,
                            link: None,
                            description: None,
                            guid: None,
                        }))
                    } else {
                        None
                    }
                },
                TagOrEvent::Tag{name, characters} => {
                    Some(TagOrPost::Tag{
                        name: name,
                        characters: characters
                    })
                }
                _ => None,
            }
        }).coalesce(|x, y| {
            match (x, y) {
                (TagOrPost::Post(post), TagOrPost::Tag{name, characters}) => {
                    let mut post = post;
                    match name.as_ref() {
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
                    Ok(TagOrPost::Post(post))
                }
                (a, b) => {
                    Err((a, b))
                }
            }
        }).filter_map(|x| {
            match x {
                TagOrPost::Post(post) => Some(post),
                TagOrPost::Tag{..} => None
            }
        }).collect_vec();

    result


}
