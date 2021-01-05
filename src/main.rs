#![feature(option_result_contains)]

use curl::easy::Easy;
use markdown::*;
use std::fs::*;
use std::io::prelude::*;
use std::io::Error;

use rocket_contrib::serve::StaticFiles;

fn main() {
    
    let prep: bool = match convert_writing_to_html() {
        Ok(ok) => true,
        Err(e) => false
    };

    let mut handle = Easy::new();
    let mut big_count = 0;
    loop {
        let mut process_id = 0;
        dbg!(big_count);
        big_count += 1;

        let child = std::thread::spawn(move || {
            process_id = std::process::id();
            rocket::ignite().mount("/", StaticFiles::from("pages")).launch();

        });
     
        let mut count = 0;
        loop {
            handle.url("http://cmoran.xyz").expect("An error occurred within url()");
            handle.connect_only(false).expect("An error occurred with connect_only()");
            count += 1;
            println!("performing connection #{}",count);
            match handle.perform() {
                Ok(()) => (),
                Err(e) => {
                    println!("{:?}",e);
                    () 
                }
            };   

            let check = match handle.response_code() {
                Ok(ok) => ok,
                Err(e) => 0
            };
            std::thread::sleep(std::time::Duration::from_millis(500));
            println!("check on site status is: {:?}",check);

            if check == 0 {
                break;
            }


        }
    }

}

fn convert_writing_to_html() -> Result<(),Error> {
    let paths = read_dir("writing/")?;
    println!("paths: {:?}",paths);
    for path in paths {

        let file_in = path.unwrap().path();
        // println!("The file is {}",file_in.clone().display());
        
        let filename = file_in.to_str().unwrap();
        println!("filename is: {:?}",filename);
        if filename.contains(&"ethics") {
            println!("filename contains \"ethics\"m skipping...");
            continue;
        } else if filename.contains(&"dmv") {
            println!("filename contains \"dmv\"m skipping...");
            continue;
        }
        
        let html_text = match file_to_html(&file_in) {
            Ok(text) => text,
            Err(e) => continue
        }; // .expect(format!("Couldn't convert {} to html",file_in.clone().display()).as_str());
        
        let filename = file_in.to_str().unwrap().split("/").collect::<Vec<&str>>().last().unwrap().split(".").collect::<Vec<&str>>();
        println!("the filename prefix is: {:?}",filename[0]);
        let file_out_path = "./pages/writing/".to_string() + filename[0] + ".html";
        let mut html_file = File::create(file_out_path)?;
        
        html_file.write_all(HTML_WRITING_PREFIX.as_bytes())?;
        html_file.write_all(html_text.as_bytes())?;
        html_file.write_all(HTML_WRITING_SUFFIX.as_bytes())?;
    }
    Ok(())
}

const HTML_WRITING_PREFIX: &str = "<!DOCTYPE html>
<html>
<head>
  <link rel=\"stylesheet\" href=\"../styles.css\">
</head>
<div class=\"container\">
<body>
";

const HTML_WRITING_SUFFIX: &str = "</body>
    </div>
    </html>
";