use std::path::{Path, PathBuf};
use std::fs::File;

use mustache;
use rocket;
use rocket::response::content::{Xml};

use util::*;
use data::Convert;
use database::Database;


fn serve_file<P : AsRef<Path>>(path: P) -> Option<File> {
    File::open(path).ok()
}

fn render_xml<P : AsRef<Path>>(path: P, data: mustache::Data) -> Xml<String> {
    Xml(render_mustache(path, data))
}

#[get("/css/<path..>")]
fn get_css(path: PathBuf) -> Option<File> {
    serve_file(Path::new("css/").join(path))
}

#[get("/ph.xsl")]
fn get_ph() -> Option<File> {
    serve_file("ph.xsl")
}

#[get("/<hunt>/index.xml", rank = 0)]
fn get_hunt(hunt: String) -> Xml<String> {
    let db = Database::new();
    let waves = db.get_waves(&hunt);
    let hunt_info = db.get_hunt(&hunt);
    println!("Waves Data! {:?}", waves);
    let data = mustache::MapBuilder::new()
        .insert_str("hunt", &hunt_info.name)
        .insert_vec("waves", |mut ws| {
            for wave in &waves {
                println!("Wave Data!");
                ws = ws.push_map(|w| wave.to_data(w))
            }
            ws
        })
        .build();
    render_xml(format!("{}/index.xml", &hunt), data)
}

#[get("/<hunt>/puzzles.xml", rank=0)]
fn get_puzzles(hunt: String) -> Xml<String> {
    let db = Database::new();
    let waves = db.get_waves(&hunt);
    let hunt_info = db.get_hunt(&hunt);
    let data = mustache::MapBuilder::new()
        .insert_str("hunt", &hunt_info.name)
        .insert_vec("waves", |mut ws| {
            for wave in &waves {
                ws = ws.push_map(|w| wave.to_data(w))
            }
            ws
        })
        .build();
    render_xml(format!("{}/puzzles.xml", &hunt), data)
}

#[get("/<hunt>/team.xml", rank=0)]
fn get_team(hunt: String) -> Xml<String> {
    let db = Database::new();
    let hunt_info = db.get_hunt(&hunt);
    let data = mustache::MapBuilder::new()
        .insert_str("hunt", &hunt_info.name)
        .insert_str("hunt_key", hunt)
        .build();
    render_xml("team.xml", data)
}

pub fn start() {
    rocket::ignite().mount("/", routes![
        get_css, get_ph,
        get_hunt, get_team, get_puzzles
            ]).launch();
}
