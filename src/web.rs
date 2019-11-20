use crate::TopListItem;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "top-composers.html")]
struct TopComposersTemplate<'a> {
    title: &'a str,
    selected_slug: &'a str,
    items: &'a Vec<(String, String, usize)>,
    menu: &'a Vec<(String, String, usize)>,
}

#[derive(Template)]
#[template(path = "composer.html")]
struct ComposerTemplate<'a> {
    composerslug: &'a str,
    selected_slug: &'a str,
    items: &'a Vec<(&'a str, &'a str, Vec<&'static TopListItem>)>,
    menu: &'a Vec<(String, String, usize)>,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    listslug: &'a str,
    selected_slug: &'a str,
    items: &'a Vec<&'a TopListItem>,
    menu: &'a Vec<(String, String, usize)>,
}

fn render_page<T: askama::Template>(s: &T) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(&s.render().unwrap()))
}

fn top_composers() -> Result<HttpResponse> {
    render_page(&TopComposersTemplate {
        title: "Top composers",
        selected_slug: "top-composers",
        items: &crate::COMPOSERS,
        menu: &crate::MENU,
    })
}

fn composer(info: web::Path<String>) -> Result<HttpResponse> {
    let name = &crate::slug_to_name(&crate::COMPOSERS, info.to_string());
    let items = crate::filter_by_composer_name(&crate::LISTS, name);
    render_page(&ComposerTemplate {
        composerslug: name.as_str(),
        selected_slug: "",
        items: &items,
        menu: &crate::MENU,
    })
}

fn list(info: web::Path<String>) -> Result<HttpResponse> {
    let name = &crate::slug_to_name(&crate::MENU, info.to_string());
    let items = crate::filter_by_list_name(&crate::LISTS, name);
    render_page(&ListTemplate {
        listslug: name.as_str(),
        selected_slug: info.as_str(),
        items: &items,
        menu: &crate::MENU,
    })
}

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
