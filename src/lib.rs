extern crate askama;

use std::error::Error;
use csv::ReaderBuilder;
use std::fs;

mod web;

#[derive(Debug)]
struct TopListItem {
    composer_name: String,
    composer_slug: String,
    work: String,
    list_name: String,
    list_slug: String,
}

fn load_top_list_from_csv() -> Result<Vec<TopListItem>, Box<dyn Error>> {
    let mut top_list_items: Vec<TopListItem> = Vec::new();
    for entry in fs::read_dir("csv")? {
        let dir_entry = entry.unwrap();
        let mut file_name = dir_entry.file_name().into_string().unwrap();
        file_name.truncate(file_name.len() - 4);
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'|')
            .from_path(&dir_entry.path())?;
        for result in rdr.records() {
            let record = result?;
            top_list_items.push(TopListItem {
                composer_name: record[0].parse()?,
                composer_slug: String::from("asdasd"),
                work: record[1].parse()?,
                list_name: file_name.clone(),
                list_slug: String::from("asdasd"),
            });
        }
    }
    Ok(top_list_items)
}

pub fn run() {
    let all_list_data = load_top_list_from_csv();
    println!("{:?}", res);
    // web::start_server();
}
