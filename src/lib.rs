//! Logic and helpers

extern crate askama;

#[macro_use]
extern crate lazy_static;

extern crate inflector;

use csv::ReaderBuilder;
use inflector::cases::kebabcase::to_kebab_case;
use std::fs;

mod web;

type ListShortForm = (String, String, usize);

/// Main source of data, works from all lists by all composers, in flat form
#[derive(Debug)]
pub struct TopListItem {
    pub composer_name: String, // Composer`s human readable name
    pub composer_slug: String, // Composer`s slug for using in URL
    pub work: String,          // Name of a single work
    pub list_name: String,     // List`s human readable name
    pub list_slug: String,     // List`s slug for using in URL
    pub position: usize,       // Work`s position in its list
}

/// Convert URL slug to human readable name: "foo-bar" -> "Foo Bar"
/// We substitute data from our main dictionary in app state, not converting strings directly
fn slug_to_name(list: &Vec<ListShortForm>, slug: String) -> String {
    // Only substitute if slug is found
    match list.iter().find(|item| item.1 == slug) {
        Some(item) => item.0.clone(),
        None => String::from(""),
    }
}

/// Convert human readable name to URL slug: "Foo Bar" -> "foo-bar"
fn name_to_slug(slug: String) -> String {
    to_kebab_case(slug.as_str())
}

/// Get all works by all composers from a single list
fn filter_by_list_name(
    items: &'static Vec<TopListItem>,
    name: &String,
) -> Vec<&'static TopListItem> {
    items
        .iter()
        .filter(|item| item.list_name.to_lowercase() == name.to_lowercase())
        .collect()
}

/// Get all works by a single composer, grouped by list
fn filter_by_composer_name(
    items: &'static Vec<TopListItem>,
    name: &String,
) -> Vec<(&'static str, &'static str, Vec<&'static TopListItem>)> {
    let mut res: Vec<(&str, &str, Vec<&'static TopListItem>)> = vec![];
    // Get flat data
    let filtered: Vec<&'static TopListItem> = items
        .iter()
        .filter(|item| item.composer_name.to_lowercase() == name.to_lowercase())
        .collect();
    // Group by list
    for item in filtered {
        match res.iter().position(|x| x.0 == item.list_name) {
            Some(x) => res[x].2.push(item),
            None => res.push((item.list_name.as_ref(), item.list_slug.as_ref(), vec![item])),
        }
    }
    // Sort by list name alphabetically
    res.sort_by(|a, b| a.0.cmp(&b.0));
    res
}

/// Load all top lists from CSV files
fn top_list_from_csv() -> Vec<TopListItem> {
    let mut all_lists: Vec<TopListItem> = Vec::new();
    // Read all files in the directory with CSV files
    for entry in fs::read_dir("csv").unwrap() {
        let dir_entry = entry.unwrap();
        // We want file names to become lists names
        let mut file_name = dir_entry.file_name().into_string().unwrap();
        // Remove ".csv" from file names
        file_name.truncate(file_name.len() - 4);
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'|')
            .from_path(&dir_entry.path())
            .unwrap();
        // Read all string from the CSV file
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

/// Build dynamic site menu based on the top lists parsed from csv
fn menu(items: &'static Vec<TopListItem>) -> Vec<ListShortForm> {
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

/// Build the list of best composers based on the top lists parsed from csv
fn top_composers(
    items: &'static Vec<TopListItem>,
    menu: &'static Vec<ListShortForm>,
) -> Vec<ListShortForm> {
    // Extract list of composers
    let mut composers: Vec<(String, String)> = items
        .iter()
        .map(|el| (el.composer_name.clone(), el.composer_slug.clone()))
        .collect();
    composers.sort();
    composers.dedup();
    // Add scores to the list of composers
    let mut composers_with_scores: Vec<ListShortForm> = composers
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
    // Sort composers by calculated score
    composers_with_scores.sort_by(|a, b| b.2.cmp(&a.2));
    composers_with_scores
}

// Create app store
lazy_static! {
    // All top lists in flat form
    static ref LISTS: Vec<TopListItem> = top_list_from_csv();
    // Site dynamic menu of top lists
    static ref MENU: Vec<ListShortForm> = menu(&LISTS);
    // List of best composers sorted by their calculated score
    static ref COMPOSERS: Vec<ListShortForm> = top_composers(&LISTS, &MENU);
}

pub fn run() {
    web::start_server();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_list_tuple() -> Vec<ListShortForm> {
        vec![
            (String::from("Orchestral Works"), String::from("orchestral-works"), 123),
            (String::from("Symphonies"), String::from("symphonies"), 123),
        ]
    }

    lazy_static! {
        static ref MOCK_TOP_LIST_ITEMS: Vec<TopListItem> = vec![
            TopListItem {
                composer_name: String::from("Beethoven"),
                composer_slug: String::from("beethoven"),
                work: String::from("Symphony No. 9 'Choral'"),
                list_name: String::from("Symphonies"),
                list_slug: String::from("symphonies"),
                position: 99,
            },
            TopListItem {
                composer_name: String::from("Sibelius"),
                composer_slug: String::from("seethoven"),
                work: String::from("Finlandia"),
                list_name: String::from("Orchestral works"),
                list_slug: String::from("orchestral-works"),
                position: 14,
            },
            TopListItem {
                composer_name: String::from("Beethoven"),
                composer_slug: String::from("beethoven"),
                work: String::from("Piano Sonata No. 32"),
                list_name: String::from("Piano Works"),
                list_slug: String::from("piano-works"),
                position: 99,
            },
        ];
    }

    #[test]
    fn test_name_to_slug() {
        assert_eq!(
            name_to_slug(String::from("Foo Bar")),
            String::from("foo-bar")
        );
        assert_eq!(name_to_slug(String::from("")), String::from(""));
        assert_eq!(name_to_slug(String::from("123")), String::from("123"));
        assert_eq!(
            name_to_slug(String::from(" foo bar ")),
            String::from("foo-bar")
        );
    }

    #[test]
    fn test_slug_to_name() {
        let mock_list =  mock_list_tuple();
        assert_eq!(slug_to_name(&mock_list, String::from("orchestral-works")), "Orchestral Works");
        assert_eq!(slug_to_name(&mock_list, String::from("not-found")), "");
    }

    #[test]
    fn test_filter_by_list_name() {
        assert_eq!(filter_by_list_name(&MOCK_TOP_LIST_ITEMS, &String::from("Symphonies")).len(), 1);
        assert_eq!(filter_by_list_name(&MOCK_TOP_LIST_ITEMS, &String::from("Unknown")).len(), 0);
    }

    #[test]
    fn test_filter_by_composer_name() {
        assert_eq!(filter_by_composer_name(&MOCK_TOP_LIST_ITEMS, &String::from("Beethoven")).len(), 2);
        assert_eq!(filter_by_composer_name(&MOCK_TOP_LIST_ITEMS, &String::from("Beethoven"))[0].2.len(), 1);
        assert_eq!(filter_by_composer_name(&MOCK_TOP_LIST_ITEMS, &String::from("Shubert")).len(), 0);
    }
}
