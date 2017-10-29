use std::path::{Path, PathBuf};
use std::fs::File;

use rocket;
use rocket::response::content::{XML};
use rocket::request::Form;

use util::*;
use data::{AddToData, build_data};
use database::Database;


fn serve_file<P : AsRef<Path>>(path: P) -> Option<File> {
    File::open(path).ok()
}

fn render_xml<P : AsRef<Path>>(path: P, data: Vec<&AddToData>) -> XML<String> {
    XML(render_mustache(path, build_data(data)))
}

#[get("/css/<path..>")]
fn get_css(path: PathBuf) -> Option<File> {
    serve_file(Path::new("css/").join(path))
}

#[get("/ph.xsl")]
fn get_ph() -> Option<File> {
    serve_file("ph.xsl")
}

#[get("/ph.js")]
fn get_js() -> Option<File> {
    serve_file("ph.js")
}

#[get("/<hunt_key>/index.xml")]
fn get_index(hunt_key: &str) -> XML<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml(format!("{}/index.xml", hunt.key), vec!(&hunt))
}

#[get("/<hunt_key>/puzzles.xml")]
fn get_puzzles(hunt_key: &str) -> XML<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let waves = db.get_waves(hunt.id);
    render_xml(format!("{}/puzzles.xml", hunt.key), vec!(&hunt, &waves))
}

#[get("/<hunt_key>/team.xml")]
fn get_team(hunt_key: &str) -> XML<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("team.xml", vec!(&hunt))
}

#[get("/<hunt_key>/view-team.xml")]
fn get_view_team(hunt_key: &str) -> XML<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("view-team.xml", vec!(&hunt))
}

#[post("/<hunt_key>/view-team.xml", data="<input>")]
fn post_view_team(hunt_key: &str, input: Form<ViewTeamInput>) -> XML<String> {
    let input = input.into_inner();
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let team = match db.get_team(hunt.id, &input.name, &input.password) {
        None => panic!("Team not found"), // TODO: error handling
        Some(team) => team
    };
    render_xml("your-team.xml", vec!(&hunt, &team))
}

#[derive(FromForm, Debug)]
struct ViewTeamInput {
    name: String,
    password: String
}

#[get("/<hunt_key>/register.xml")]
fn get_register(hunt_key: &str) -> XML<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("register.xml", vec!(&hunt))
}

#[post("/<hunt_key>/register.xml", data="<input>")]
fn post_register(hunt_key: &str, input: Form<RegistrationInput>) -> XML<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let input = input.into_inner();
    if input.password != input.password_verify {
        panic!("Passwords do not match");
    }
    println!("Team Registration: {:?}", input);
    render_xml("view-team.xml", vec!(&hunt))
}

#[derive(FromForm, Debug)]
struct RegistrationInput {
    name: String,
    password: String,
    password_verify: String,
    member_name: String,
    member_email: String,
}


pub fn start() {
    rocket::ignite().mount("/", routes![
        get_css, get_ph, get_js,
        get_index, get_puzzles, get_team, get_register, get_view_team,
        post_register, post_view_team
    ]).launch();
}
