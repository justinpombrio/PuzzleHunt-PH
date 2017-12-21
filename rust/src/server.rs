use std::path::{Path, PathBuf};
use std::fs::File;

use rocket;
use rocket::response::content::Xml;
use rocket::request::Form;

use util::*;
use data::{AddToData, build_data};
use database::Database;
use logic::{ViewTeam, Register};


fn serve_file<P : AsRef<Path>>(path: P) -> Option<File> {
    File::open(path).ok()
}

fn render_xml<P : AsRef<Path>>(path: P, data: Vec<&AddToData>) -> Xml<String> {
    Xml(render_mustache(path, build_data(data)))
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

#[get("/<hunt_key>/puzzles.xml", rank=0)]
fn get_puzzles(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let waves = db.get_waves(hunt.id);
    render_xml(format!("{}/puzzles.xml", hunt.key), vec!(&hunt, &waves))
}

#[get("/<hunt_key>/team.xml", rank=0)]
fn get_team(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let waves = db.get_waves(hunt.id);
    render_xml("team.xml", vec!(&hunt, &waves))
}

#[get("/<hunt_key>/index.xml", rank=0)]
fn get_hunt(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml(format!("{}/index.xml", hunt.key), vec!(&hunt))
}

#[get("/<hunt_key>/view-team.xml", rank=0)]
fn get_view_team(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("view-team.xml", vec!(&hunt))
}

#[post("/<hunt_key>/view-team.xml", data="<form>")]
fn post_view_team(hunt_key: String, form: Form<ViewTeam>) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let team = match form.get().view_team(hunt.id) {
        Ok(team) => team,
        Err(msg) => panic!("{}", msg)
    };
    println!("team: {:?}", &team);
    render_xml("your-team.xml", vec!(&hunt, &team))
}

#[get("/<hunt_key>/register.xml", rank=0)]
fn get_register(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("register.xml", vec!(&hunt))
}

#[post("/<hunt_key>/register.xml", data="<form>")]
fn post_register(hunt_key: String, form: Form<Register>) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let form = form.into_inner();
    if form.password != form.password_verify {
        panic!("Passwords do not match");
    }
    println!("Team Registration: {:?}", form);
    render_xml("your-team.xml", vec!(&hunt))
}


pub fn start() {
    rocket::ignite().mount("/", routes![
        get_css, get_ph, get_js,
        get_hunt, get_puzzles, get_team, get_register, get_view_team,
        post_register, post_view_team
    ]).launch();
}
