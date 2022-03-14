use std::error::Error;
use std::fs;
use std::iter::zip;
use std::path::{Path, PathBuf};

use aho_corasick::AhoCorasick;

fn main() {
    let path = PathBuf::from("geospatial").join("data");
    let image_layers = create_image_layers(path).unwrap();
    println!("{:#?}", image_layers);
}

fn create_image_layers(path: PathBuf) -> Result<String, Box<dyn Error>> {
    let (datasets, summaries) = get_dataset_info(path).unwrap();
    println!("{:#?}", &datasets);
    let mut iter = zip(&datasets, &summaries).for_each(|(x, y)| println!("{:?},{:?}", x, y));

    let path = Path::new("geospatial")
        .join("templates")
        .join("map_image")
        .with_extension("js");
    let base_text = fs::read_to_string(path).unwrap();

    let mut image_layers = String::new();
    for i in 0..datasets.len() {
        let summary = &summaries[i];
        let min_x = summary.bounds.min_x.to_string();
        let max_x = summary.bounds.max_x.to_string();
        let min_y = summary.bounds.min_y.to_string();
        let max_y = summary.bounds.max_y.to_string();
        let name = datasets[i].to_str().unwrap().to_string();

        let patterns = &[
            "[data:dataset_name]",
            "[data:min_x]",
            "[data:max_x]",
            "[data:min_y]",
            "[data:max_y]",
        ];

        let replace_with = &[&name, &min_x, &max_x, &min_y, &max_y];
        let ac = AhoCorasick::new(patterns);
        let layer = ac.replace_all(&base_text, replace_with);
        image_layers += &layer;
    }

    Ok(image_layers)
}

fn get_dataset_info(
    path: impl Into<PathBuf>,
) -> Result<(Vec<PathBuf>, Vec<Summary>), Box<dyn Error>> {
    let path = path.into();
    if path.exists() != true {
        panic!("path does not exist");
    }
    let mut len = 10;
    let mut datasets: Vec<PathBuf> = Vec::with_capacity(len);
    let mut summaries: Vec<Summary> = Vec::with_capacity(len);
    if path.is_dir() {
        for entry in fs::read_dir(&path)? {
            let entry = entry?.path();
            // Iterate over the datasets
            if entry.is_dir() {
                let json_path = entry.join("summary.json");
                let json_text = match fs::read_to_string(json_path) {
                    Ok(text) => text,
                    Err(_e) => continue,
                };
                let s: Summary = serde_json::from_str(&json_text)?;
                let dataset: PathBuf = match entry.file_stem() {
                    Some(stem) => stem.to_os_string().into(),
                    None => continue,
                };
                datasets.push(dataset);
                summaries.push(s);
            }
        }
    }

    Ok((datasets, summaries))
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Center {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub title: String,
    pub description: String,
    pub bounds: Bounds,
    pub center: Center,
}
