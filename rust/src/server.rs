use std::path::{Path, PathBuf};
use std::fs::File;

use mustache;
use rocket;
use rocket::response::content::{XML};

use util::*;


fn serve_file<P : AsRef<Path>>(path: P) -> Option<File> {
    File::open(path).ok()
}

fn render_xml<P : AsRef<Path>>(path: P, data: mustache::Data) -> XML<String> {
    XML(render_mustache(path, data))
}

#[get("/css/<path..>")]
fn get_css(path: PathBuf) -> Option<File> {
    serve_file(Path::new("css/").join(path))
}

#[get("/ph.xsl")]
fn get_ph() -> Option<File> {
    serve_file("ph.xsl")
}

#[get("/index.xml")]
fn get_index() -> XML<String> {
    let data = mustache::MapBuilder::new()
        .insert_str("hunt", "The <b>best</b> hunt")
        .insert_vec("waves", |waves| {
            waves
                .push_map(|wave| {
                    wave
                        .insert_str("name", "One and only Wave")
                        .insert_vec("puzzles", |puzzles| {
                            puzzles
                                .push_map(|puzzle| {
                                    puzzle
                                        .insert_str("name", "First \"Puzzle")
                                        .insert_str("key", "puzzle1")
                                })
                                .push_map(|puzzle| {
                                    puzzle
                                        .insert_str("name", "Second Puzzle")
                                        .insert_str("key", "puzzle2")
                                })
                        })
                })
        })
        .build();
    render_xml("index.xml", data)
}

pub fn start() {
    rocket::ignite().mount("/", routes![get_css, get_ph, get_index]).launch();
}
