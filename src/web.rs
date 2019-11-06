use actix_web::{web, App, HttpResponse, HttpServer, Result};
use askama::Template;
use crate::TopListItem;

#[derive(Template)]
#[template(path = "top-composers.html")]
struct TopComposersTemplate<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "composer.html")]
struct ComposerTemplate<'a> {
    title: &'a str,
    composerid: &'a u32,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    title: &'a str,
    listslug: &'a str,
}

struct AppStore<'a> {
    all_lists: &'a [TopListItem],
}

fn render_page<T: askama::Template>(s: &T) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(&s.render().unwrap()))
}

fn top_composers() -> Result<HttpResponse> {
    render_page(&TopComposersTemplate {
        title: "Best composers",
    })
}

fn composer(info: web::Path<u32>) -> Result<HttpResponse> {
    render_page(&ComposerTemplate {
        title: "Best composers",
        composerid: &info,
    })
}

fn list(info: web::Path<String>) -> Result<HttpResponse> {
    render_page(&ListTemplate {
        title: "Best composers",
        listslug: &info,
    })
}

pub fn start_server() {
    HttpServer::new(move || {
        println!("{:?}", *crate::LISTS);
        App::new()
            //.data(AppStore{ all_lists: all_lists.as_slice() })
            .route("/", web::get().to(top_composers))
            .route("/composer/{composerid}", web::get().to(composer))
            .route("/{listslug}", web::get().to(list))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
