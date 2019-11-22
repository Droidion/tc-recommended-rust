//! All about web server

use crate::{TopListItem, ListShortForm};
use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use askama::Template;

/// Payload for rendering sorted list of best composers page
#[derive(Template)]
#[template(path = "top-composers.html")]
struct TopComposersTemplate<'a> {
    title: &'a str,                          // Page title
    selected_slug: &'a str,                  // Current page slug, for showing selected menu item
    items: &'a Vec<ListShortForm>, // Sorted list of best composers
    menu: &'a Vec<ListShortForm>,  // Dynamic part of site menu
}

/// Payload for rendering a composer page
#[derive(Template)]
#[template(path = "composer.html")]
struct ComposerTemplate<'a> {
    composer_name: &'a str, // Human readable composer name
    selected_slug: &'a str, // Current page slug, for showing selected menu item
    items: &'a Vec<(&'a str, &'a str, Vec<&'static TopListItem>)>, // Works of a single composer grouped by lists
    menu: &'a Vec<ListShortForm>,                        // Dynamic part of site menu
}

/// Payload for rendering a single list page
#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    list_name: &'a str,                     // Human readable list name
    selected_slug: &'a str,                 // Current page slug, for showing selected menu item
    items: &'a Vec<&'a TopListItem>,        // Works of single list
    menu: &'a Vec<ListShortForm>, // Dynamic part of site menu
}

/// Helper for rendering a page with some data
fn render_page<T: askama::Template>(s: &T) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(&s.render().unwrap()))
}

/// Render sorted list of best composers
fn top_composers() -> Result<HttpResponse> {
    render_page(&TopComposersTemplate {
        title: "Top composers",
        selected_slug: "top-composers",
        items: &crate::COMPOSERS,
        menu: &crate::MENU,
    })
}

/// Render all works by a single composer grouped by lists
fn composer(info: web::Path<String>) -> Result<HttpResponse> {
    let name = &crate::slug_to_name(&crate::COMPOSERS, info.to_string());
    let items = crate::filter_by_composer_name(&crate::LISTS, name);
    render_page(&ComposerTemplate {
        composer_name: name.as_str(),
        selected_slug: "",
        items: &items,
        menu: &crate::MENU,
    })
}

/// Render page with works from a single list
fn list(info: web::Path<String>) -> Result<HttpResponse> {
    let name = &crate::slug_to_name(&crate::MENU, info.to_string());
    let items = crate::filter_by_list_name(&crate::LISTS, name);
    render_page(&ListTemplate {
        list_name: name.as_str(),
        selected_slug: info.as_str(),
        items: &items,
        menu: &crate::MENU,
    })
}

/// Start web server
pub fn start_server() {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .service(fs::Files::new("/static", "./static"))
            .route("/", web::get().to(top_composers))
            .route("/top-composers", web::get().to(top_composers))
            .route("/composer/{composerslug}", web::get().to(composer))
            .route("/{listslug}", web::get().to(list))
    })
    .bind("0.0.0.0:8088")
    .unwrap()
    .run()
    .unwrap();
}
