use std::error::Error;
use std::fs;
use std::iter::zip;
use std::path::{Path, PathBuf};

use geojson::{feature::Id, *};
use serde_json::Value;

fn main() {
    let path = PathBuf::from("geospatial").join("data");
    let collection = create_feature_collection(path).unwrap();
    println!("{:#?}", collection);
}

fn create_feature_collection(path: PathBuf) -> Result<FeatureCollection, Box<dyn Error>> {
    let (datasets, summaries) = get_dataset_info(path).unwrap();
    let mut iter = zip(&datasets, &summaries).for_each(|(x, y)| println!("{:?},{:?}", x, y));

    let mut feature_collection = FeatureCollection {
        bbox: None,
        features: vec![],
        foreign_members: None,
    };
    for i in 0..summaries.len() {
        let summary = &summaries[i];
        let mut geometry = Geometry::new(geojson::Value::Point(vec![
            summary.center.lat,
            summary.center.lon,
        ]));
        // geometry.set_property(("type","Point"));

        let mut feature = Feature {
            bbox: None,
            geometry: Some(geometry),
            id: None,
            properties: None,
            foreign_members: None,
        };
        feature.set_property(
            "description",
            format!("<h1>{}</h1><p>{}</p>", summary.title, summary.description),
        );

        feature_collection.features.push(feature);
    }

    Ok(feature_collection)
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
