extern crate askama;

#[macro_use]
extern crate lazy_static;

use csv::ReaderBuilder;
use std::fs;

mod web;

#[derive(Debug)]
pub struct TopListItem {
    pub composer_name: String,
    pub composer_slug: String,
    pub work: String,
    pub list_name: String,
    pub list_slug: String,
}

fn slug_to_name(slug: String) -> String {
    slug.replace("-", " ")
}

fn filter_by_list_name(items: &'static Vec<TopListItem>, name: String) -> Vec<&'static TopListItem> {
    items.iter().filter(|item| item.list_name.to_lowercase() == name.to_lowercase()).collect()
}

fn filter_by_composer_name(items: &'static Vec<TopListItem>, name: String) -> Vec<&'static TopListItem> {
    items.iter().filter(|item| item.composer_name.to_lowercase() == name.to_lowercase()).collect()
}

fn load_top_list_from_csv() -> Vec<TopListItem> {
    let mut all_lists: Vec<TopListItem> = Vec::new();
    for entry in fs::read_dir("csv").unwrap() {
        let dir_entry = entry.unwrap();
        let mut file_name = dir_entry.file_name().into_string().unwrap();
        file_name.truncate(file_name.len() - 4);
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'|')
            .from_path(&dir_entry.path()).unwrap();
        for result in rdr.records() {
            let record = result.unwrap();
            all_lists.push(TopListItem {
                composer_name: record[0].parse().unwrap(),
                composer_slug: String::from("asdasd"),
                work: record[1].parse().unwrap(),
                list_name: file_name.clone(),
                list_slug: String::from("asdasd"),
            });
        }
    }
    all_lists
}

lazy_static! {
    static ref LISTS: Vec<TopListItem> = load_top_list_from_csv();
}

pub fn run() {
    web::start_server();
}
