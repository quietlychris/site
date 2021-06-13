use markdown::*;
use std::fs::*;
use std::io::prelude::*;
use std::io::Error;

use rocket::fs::{relative, FileServer};

#[rocket::launch]
fn site() -> _ {
    // env_logger::init();
    convert_writing_to_html("static","writing").expect("An error occurred while converting markdown files to html");
    compile_wasm().expect("There was an error compiling the wasm pages");
    
    let use_wasm = true;

    let site = match use_wasm {
        true => {
            rocket::build().mount("/", FileServer::from(relative!("static")).rank(0) )
                .mount("/wasm", FileServer::from(relative!("wasm")).rank(1) )
        },
        false => {
            rocket::build().mount("/", FileServer::from(relative!("static")))
        }
    };
    
    site
}

fn convert_writing_to_html(base_dir: &str, writing_dir: &str) -> Result<(), Error> {
    let paths = read_dir(writing_dir).unwrap();
    println!("- Converting markdown files in {:?} to html:", paths);
    for path in paths {
        let file_in = path.unwrap().path();
        // println!("The file is {}", file_in.clone().display());
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

        // println!("the filename prefix is: {:?}", filename[0]);
        let file_out_path = base_dir.to_owned() + "/" + &writing_dir.to_owned() + "/" + filename[0] + ".html";
        println!("    - Writing {:?} to \"{}\"",file_in, file_out_path);
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

    let wasm = Path::new("./wasm/basic/");
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

    let home = Path::new("../..");
    assert!(env::set_current_dir(&home).is_ok());
    
    //println!(
    //   "Successfully changed working directory to {}!",
    //  home.display()
    //);

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
