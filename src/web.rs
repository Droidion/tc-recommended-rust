use crate::TopListItem;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "top-composers.html")]
struct TopComposersTemplate<'a> {
    title: &'a str,
    menu: &'a Vec<(String, String)>,
}

#[derive(Template)]
#[template(path = "composer.html")]
struct ComposerTemplate<'a> {
    title: &'a str,
    composerslug: &'a str,
    items: &'a Vec<(&'a str, Vec<&'static TopListItem>)>,
    menu: &'a Vec<(String, String)>,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    title: &'a str,
    listslug: &'a str,
    items: &'a Vec<&'a TopListItem>,
    menu: &'a Vec<(String, String)>,
}

fn render_page<T: askama::Template>(s: &T) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(&s.render().unwrap()))
}

fn top_composers() -> Result<HttpResponse> {
    render_page(&TopComposersTemplate {
        title: "Best composers",
        menu: &crate::MENU,
    })
}

fn composer(info: web::Path<String>) -> Result<HttpResponse> {
    let items =
        crate::filter_by_composer_name(&crate::LISTS, crate::slug_to_name(info.to_string()));
    render_page(&ComposerTemplate {
        title: "Best composers",
        composerslug: &info,
        items: &items,
        menu: &crate::MENU,
    })
}

fn list(info: web::Path<String>) -> Result<HttpResponse> {
    let items = crate::filter_by_list_name(&crate::LISTS, crate::slug_to_name(info.to_string()));
    render_page(&ListTemplate {
        title: "Best composers",
        listslug: &info,
        items: &items,
        menu: &crate::MENU,
    })
}

pub fn start_server() {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(top_composers))
            .route("/composer/{composerslug}", web::get().to(composer))
            .route("/{listslug}", web::get().to(list))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
