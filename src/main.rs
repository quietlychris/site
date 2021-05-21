#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use markdown::*;
use std::fs::*;
use std::io::prelude::*;
use std::io::Error;

use rocket_contrib::serve::StaticFiles;

#[get("/wasm_map.html")]
fn wasm_map() -> String {
    format!("Hello, world")
}

fn main() {
    convert_writing_to_html().expect("An error occurred while converting markdown files to html");
    compile_wasm().expect("There was an error compiling the wasm pages");
    rocket::ignite()
        .mount("/", StaticFiles::from("static_pages"))
        .mount("/wasm", StaticFiles::from("wasm_pages").rank(2))
        .launch();
}

fn convert_writing_to_html() -> Result<(), Error> {
    let paths = read_dir("writing/").unwrap();
    println!("paths: {:?}", paths);
    for path in paths {
        let file_in = path.unwrap().path();
        println!("The file is {}", file_in.clone().display());
        let html_text = file_to_html(&file_in)
            .expect(format!("Couldn't convert {} to html", file_in.clone().display()).as_str());

        let filename = file_in
            .to_str()
            .unwrap()
            .split("/")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>();

        println!("the filename prefix is: {:?}", filename[0]);
        let file_out_path = "./static_pages/writing/".to_string() + filename[0] + ".html";
        let mut html_file = File::create(file_out_path)?;

        html_file.write_all(HTML_WRITING_PREFIX.as_bytes())?;
        html_file.write_all(html_text.as_bytes())?;
        html_file.write_all(HTML_WRITING_SUFFIX.as_bytes())?;
    }
    Ok(())
}

fn compile_wasm() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use std::path::Path;
    use std::process::Command;

    let wasm = Path::new("./wasm_pages/basic/");
    assert!(env::set_current_dir(&wasm).is_ok());
    println!("Successfully changed working directory to {}!", wasm.display());

    let compilation = Command::new("wasm-pack")
        .arg("build")
        .arg("--target")
        .arg("web")
        .output()
        .expect("failed to execute process");
    
    compilation.stdout; //.expect("Failed to compile");

    let home = Path::new("../..");
    assert!(env::set_current_dir(&home).is_ok());
    println!("Successfully changed working directory to {}!", home.display());


    Ok(())
}

const HTML_WRITING_PREFIX: &str = "<!DOCTYPE html>
<html>
<head>
  <link rel=\"stylesheet\" href=\"../styles.css\">
  <script async defer data-domain=\"cmoran.xyz\" src=\"https://plausible.io/js/plausible.js\"></script>
</head>
<div class=\"container\">
<body>
<ul>
 <li><a href=\"/index.html\">Home</a></li>
 <li><a href=\"https://github.com/quietlychris\">GitHub</a></li>
 <li><a href=\"/writing.html\">Writing</a></li>
 <li><a href=\"/cmoran.pdf\">Resumé</a></li>
</ul>
";

const HTML_WRITING_SUFFIX: &str = "</body>
    </div>
    </html>
";
