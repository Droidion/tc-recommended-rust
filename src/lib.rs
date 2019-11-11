extern crate askama;

#[macro_use]
extern crate lazy_static;

extern crate inflector;

use inflector::cases::kebabcase::to_kebab_case;
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
    pub position: usize,
}

fn composer_slug_to_name(slug: String) -> String {
    match COMPOSERS.iter().find(|composer| composer.1 == slug) {
        Some(composer) => composer.0.clone(),
        None => String::from("")
    }
}
fn list_slug_to_name(slug: String) -> String {
    match MENU.iter().find(|item| item.1 == slug) {
        Some(item) => item.0.clone(),
        None => String::from("")
    }
}
fn name_to_slug(slug: String) -> String {
    to_kebab_case(slug.as_str())
}

fn filter_by_list_name(
    items: &'static Vec<TopListItem>,
    name: String,
) -> Vec<&'static TopListItem> {
    items
        .iter()
        .filter(|item| item.list_name.to_lowercase() == name.to_lowercase())
        .collect()
}

fn filter_by_composer_name(
    items: &'static Vec<TopListItem>,
    name: String,
) -> Vec<(&str, &str, Vec<&'static TopListItem>)> {
    let mut res: Vec<(&str, &str, Vec<&'static TopListItem>)> = vec![];
    let filtered: Vec<&'static TopListItem> = items
        .iter()
        .filter(|item| item.composer_name.to_lowercase() == name.to_lowercase())
        .collect();
    for item in filtered {
        match res.iter().position(|x| x.0 == item.list_name) {
            Some(x) => res[x].2.push(item),
            None => res.push((item.list_name.as_ref(), item.list_slug.as_ref(), vec![item])),
        }
    }
    res.sort_by(|a, b| a.0.cmp(&b.0));
    res
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
            .from_path(&dir_entry.path())
            .unwrap();
        for (i, result) in rdr.records().enumerate() {
            let record = result.unwrap();
            all_lists.push(TopListItem {
                composer_name: record[0].parse().unwrap(),
                composer_slug: name_to_slug(record[0].parse().unwrap()),
                work: record[1].parse().unwrap(),
                list_name: file_name.clone(),
                list_slug: name_to_slug(file_name.clone()),
                position: i + 1,
            });
        }
    }
    all_lists
}

fn get_menu(items: &'static Vec<TopListItem>) -> Vec<(String, String, usize)> {
    let mut menu: Vec<(String, String)> = items
        .iter()
        .map(|el| (el.list_name.clone(), el.list_slug.clone()))
        .collect();
    menu.sort();
    menu.dedup();
    menu.iter()
        .map(|el| {
            let filtered: Vec<&'static TopListItem> =
                items.iter().filter(|item| item.list_slug == el.1).collect();
            (el.0.clone(), el.1.clone(), filtered.len())
        })
        .collect()
}

fn get_top_composers(
    items: &'static Vec<TopListItem>,
    menu: &'static Vec<(String, String, usize)>,
) -> Vec<(String, String, usize)> {
    let mut composers: Vec<(String, String)> = items
        .iter()
        .map(|el| (el.composer_name.clone(), el.composer_slug.clone()))
        .collect();
    composers.sort();
    composers.dedup();
    let mut composers_with_scores: Vec<(String, String, usize)> = composers
        .iter()
        .map(|el| {
            // initialize score
            let mut score: usize = 0;
            // find all items for this composer
            let items_for_composer: Vec<&'static TopListItem> = items
                .iter()
                .filter(|item| item.composer_slug == el.1)
                .collect();
            // calc and score based on this list length
            for composer_item in items_for_composer {
                match menu
                    .iter()
                    .find(|menu_item| menu_item.1 == composer_item.list_slug)
                {
                    Some(x) => score += 1000 * x.2 / composer_item.position,
                    None => (),
                }
            }
            (el.0.clone(), el.1.clone(), score)
        })
        .collect();
    composers_with_scores.sort_by(|a, b| b.2.cmp(&a.2));
    composers_with_scores
}

lazy_static! {
    static ref LISTS: Vec<TopListItem> = load_top_list_from_csv();
    static ref MENU: Vec<(String, String, usize)> = get_menu(&LISTS);
    static ref COMPOSERS: Vec<(String, String, usize)> = get_top_composers(&LISTS, &MENU);
}

pub fn run() {
    web::start_server();
}
