#![feature(custom_derive, plugin)]

#![plugin(postgres_macros)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate xml;
extern crate itertools;
extern crate postgres;
extern crate serde;

pub mod rss_parser;
pub mod db;
pub mod models;
