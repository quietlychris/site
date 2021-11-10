#[macro_use]
extern crate rocket;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::content;

use markdown::*;
use std::path::{Path, PathBuf};

// Expands to async main function
#[rocket::launch]
fn site() -> _ {
    // env_logger::init();

    // compile_wasm().expect("There was an error compiling the wasm pages");

    rocket::build()
        .mount("/", routes![home, styles, index_pages, resume, writing])
        .mount("/data", routes![dataset_index_pages, dataset_content])
        .mount("/sandbox", FileServer::from("sandbox")) // Experimental, don't care much about formatting
}

#[get("/")] // <- route attribute
fn home() -> content::Html<String> {
    let page_name = "./assets/index.html".to_string();
    let content = match std::fs::read_to_string(&page_name) {
        Ok(text) => text,
        Err(e) => format!("Error: {:?} for {}", e, &page_name),
    };
    format_content(content)
}

#[get("/styles.css")]
fn styles() -> content::Css<String> {
    let page_name = "./assets/styles.css".to_string();
    let content = match std::fs::read_to_string(&page_name) {
        Ok(text) => text,
        Err(e) => format!("Error: {:?} for {}", e, &page_name),
    };
    content::Css(content)
}

#[get("/<index_page>")]
fn index_pages(index_page: &str) -> content::Html<String> {
    let path = Path::new(index_page).join("index.html");
    let content = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) => format!("Error: {:?} for {:?}", e, &path),
    };
    format_content(content)
}

#[get("/cmoran.pdf")]
async fn resume() -> Option<NamedFile> {
    NamedFile::open(Path::new("assets").join("cmoran.pdf"))
        .await
        .ok()
}

#[get("/writing/<page>")]
fn writing(page: &str) -> content::Html<String> {
    let path = Path::new("writing").join(page).with_extension("md");
    // dbg!(&path);

    let content = match file_to_html(&path) {
        Ok(text) => text,
        Err(e) => format!("\nError converting {:?} to html: {:?}", path, e),
    };
    format_content(content)
}

#[get("/<category>")]
async fn dataset_index_pages(category: PathBuf) -> content::Html<String> {
    let mut path = Path::new("data").join(category);
    if path.is_dir() {
        path.push("index.html");
    }
    dbg!(&path);
    let content = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) => format!("Error: {:?} for {:?}", e, &path),
    };
    format_content(content)
}

#[get("/<category>/<payload..>", rank = 2)]
async fn dataset_content(category: PathBuf, payload: PathBuf) -> Option<NamedFile> {
    let path = Path::new("data").join(category).join(payload);
    NamedFile::open(path).await.ok()
}

/// Experimental function for compiling the Leaflet WebAssembly map prior to serving the site
fn compile_wasm() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use std::process::Command;

    let wasm = Path::new("./sandbox/wasm/map/");
    assert!(env::set_current_dir(&wasm).is_ok());

    //println!(
    //    "Successfully changed working directory to {}!",
    //    wasm.display()
    //);

    let compilation = Command::new("wasm-pack")
        .arg("build")
        .arg("--target")
        .arg("web")
        .output()
        .expect("failed to execute wasm-pack build process");

    compilation.stdout; //.expect("Failed to compile");

    let home = Path::new("../../..");
    assert!(env::set_current_dir(&home).is_ok());

    //println!(
    //   "Successfully changed working directory to {}!",
    //  home.display()
    //);

    Ok(())
}

/// Apply site-wide formatting rules to a raw html page
fn format_content(content: String) -> content::Html<String> {
    let prefix = match std::fs::read_to_string("assets/WRITING_PREFIX.html") {
        Ok(text) => text,
        Err(e) => format!("Error getting html formatting PREFIX: {:?}", e),
    };
    let suffix = match std::fs::read_to_string("assets/WRITING_SUFFIX.html") {
        Ok(text) => text,
        Err(e) => format!("Error getting html formatting SUFFIX: {:?}", e),
    };
    let page = prefix + &content + &suffix;
    content::Html(page)
}
