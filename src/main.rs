#[macro_use]
extern crate rocket;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::content;

use aho_corasick::AhoCorasick;

use comrak::adapters::SyntaxHighlighterAdapter;
use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

mod maplibre;
use crate::maplibre::*;

static ERROR_MSG: &'static str = "Yikes 😬 There was an error processing your request for: ";

// Expands to async main function
#[rocket::launch]
fn site() -> _ {
    rocket::build()
        .mount("/", routes![home, styles, index_pages, resume, writing])
        .mount("/writing/assets", FileServer::from("writing/assets"))
        .mount("/data", routes![dataset_index_pages, dataset_content])
        .mount("/geospatial", routes![map, map_template_js, data])
        .mount("/sandbox", FileServer::from("sandbox")) // Experimental, don't care much about formatting
}

#[get("/<category>/<payload..>", rank = 2)]
async fn dataset_content(category: PathBuf, payload: PathBuf) -> Option<NamedFile> {
    let path = Path::new("data").join(category).join(payload);
    NamedFile::open(path).await.ok()
}

#[get("/templates/map.js", rank = 2)]
async fn map_template_js() -> content::RawJavaScript<String> {
    let path = Path::new("geospatial")
        .join("templates")
        .join("map")
        .with_extension("js");
    let base_text = fs::read_to_string(path).unwrap();

    let patterns = &[
        "[data:feature_collection]",
        "[data:all_images]",
        "[setting:toggles]",
    ];

    let data_path = PathBuf::from("geospatial").join("data");
    let feature_collection = create_feature_collection(&data_path).unwrap();
    let image_layers = create_image_layers(&data_path).unwrap();

    let toggles = maplibre::list_toggles();

    let j = serde_json::to_string_pretty(&feature_collection).unwrap();

    let replace_with = &[j, image_layers, toggles];
    let ac = AhoCorasick::new(patterns);
    let page = ac.replace_all(&base_text, replace_with);

    content::RawJavaScript(page)
}

#[get("/", rank = 0)]
async fn map() -> Option<NamedFile> {
    let path = Path::new("geospatial")
        .join("templates")
        .join("map")
        .with_extension("html");
    NamedFile::open(path).await.ok()
}

#[get("/data/<dataset>/<filename>", rank = 2)]
async fn data(dataset: PathBuf, filename: PathBuf) -> Option<NamedFile> {
    let path = Path::new("geospatial")
        .join("data")
        .join(dataset)
        .join(filename);
    NamedFile::open(path).await.ok()
}

#[get("/<category>")]
async fn dataset_index_pages(category: PathBuf) -> content::RawHtml<String> {
    let mut path = Path::new("data").join(category);
    if path.is_dir() {
        path.join("index").with_extension("html");
    }
    dbg!(&path);
    let content = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) => format!("Error: {:?} for {:?}", e, &path),
    };
    format_content(content)
}

#[get("/")] // <- route attribute
fn home() -> content::RawHtml<String> {
    let page_name = "./assets/index.html".to_string();
    let content = match std::fs::read_to_string(&page_name) {
        Ok(text) => text,
        Err(e) => format!("Error: {:?} for {}", e, &page_name),
    };
    format_content(content)
}

#[get("/<index_page>", rank = 1)]
fn index_pages(index_page: &str) -> content::RawHtml<String> {
    let path = Path::new(index_page).join("index.html");
    let content = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) => format!("<p>{} {:?}</p>", ERROR_MSG, &path),
    };
    format_content(content)
}

#[get("/cmoran.pdf")]
async fn resume() -> Option<NamedFile> {
    NamedFile::open(Path::new("assets").join("cmoran.pdf"))
        .await
        .ok()
}

#[get("/styles.css")]
fn styles() -> content::RawCss<String> {
    let page_name = "./assets/styles.css".to_string();
    let content = match std::fs::read_to_string(&page_name) {
        Ok(text) => text,
        Err(e) => format!("Error: {:?} for {}", e, &page_name),
    };
    content::RawCss(content)
}

#[get("/writing/<page>")]
fn writing(page: &str) -> content::RawHtml<String> {
    let path = Path::new("writing").join(page).with_extension("md");
    // dbg!(&path);

    // Using Comrak to render markdown, including a number of extensions
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;
    options.render.github_pre_lang = true;
    options.render.hardbreaks = true;
    options.extension.footnotes = true;

    let mut plugins = ComrakPlugins::default();
    let adapter = SyntectAdapter::new("base16-mocha.dark");
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let content = match std::fs::read_to_string(&path) {
        Ok(text) => markdown_to_html_with_plugins(&text, &options, &plugins),
        Err(e) => format!("\nError converting {:?} to html: {:?}", path, e),
    };
    format_content(content)
}

/// Apply site-wide formatting rules to a raw html page
fn format_content(content: String) -> content::RawHtml<String> {
    let template = match std::fs::read_to_string("assets/page.html") {
        Ok(text) => text,
        Err(e) => format!("Error getting html formatting PREFIX: {:?}", e),
    };

    let patterns = &["[fn:insert_content]"];

    let replace_with = &[content];
    let ac = AhoCorasick::new(patterns);
    let page = ac.replace_all(&template, replace_with);

    content::RawHtml(page)
}
