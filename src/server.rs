use std::path::{Path, PathBuf};
use std::fs::File;

use rocket;
use rocket::response::content::Xml;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket::http::Cookies;

use util::*;
use data::{AddToData, build_data};
use database::Database;
use forms::{SignInForm, RegisterForm, UpdateTeamForm};


fn serve_file<P : AsRef<Path>>(path: P) -> Option<File> {
    File::open(path).ok()
}

fn render_xml<P : AsRef<Path>>(path: P, data: Vec<&AddToData>) -> Xml<String> {
    Xml(render_mustache(path, build_data(data)))
}


// Resources //

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


// Index //

#[get("/")]
fn get_index() -> Xml<String> {
    let db = Database::new();
    let hunts = db.get_hunts();
    render_xml("index.xml", vec!(&hunts))
}


// Hunts //

#[get("/<hunt_key>/signin.xml", rank=0)]
fn get_signin(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("signin.xml", vec!(&hunt))
}

#[post("/<hunt_key>/signin.xml", rank=0, data="<form>")]
fn post_signin(hunt_key: String, mut cookies: Cookies, form: Form<SignInForm>) -> Redirect {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let form = form.into_inner();
    if db.signin_team(&mut cookies, hunt.id, &form.name, &form.password) {
        Redirect::to("your-team.xml")
    } else {
        panic!("Failed to sign in.") // TODO: error handling
    }
}

#[get("/<hunt_key>/signout.xml", rank=0)]
fn get_signout(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("signout.xml", vec!(&hunt))
}

#[allow(unused_variables)]
#[post("/<hunt_key>/signout.xml")]
fn post_signout(hunt_key: String, mut cookies: Cookies) -> Redirect {
    let db = Database::new();
    db.signout_team(&mut cookies);
    Redirect::to("/<hunt_key>")
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
    render_xml("team.xml", vec!(&hunt))
}

#[get("/<hunt_key>", rank=0)]
fn get_hunt_base(hunt_key: String) -> Redirect {
    Redirect::to(&format!("/{}/index.xml", hunt_key))
}

#[get("/<hunt_key>/index.xml", rank=0)]
fn get_hunt(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml(format!("{}/index.xml", hunt.key), vec!(&hunt))
}
/*
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
        Err(msg) => panic!("{}", msg) // TODO: error handling
    };
    println!("team: {:?}", &team);
    render_xml("your-team.xml", vec!(&hunt, &team))
}
*/
#[get("/<hunt_key>/register.xml", rank=0)]
fn get_register(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("register.xml", vec!(&hunt))
}

#[post("/<hunt_key>/register.xml", data="<form>")]
fn post_register(hunt_key: String, form: Form<RegisterForm>) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let form = form.into_inner();
    let team = match db.register(hunt.id, &form) {
        Ok(team) => team,
        Err(msg) => panic!("{}", msg) // TODO: error handling
    };
    render_xml("your-team.xml", vec!(&hunt, &team))
}

#[get("/<hunt_key>/your-team.xml", rank=0)]
fn get_your_team(hunt_key: String, mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let team = match db.signedin_team(&mut cookies, hunt.id) {
        Some(team) => team,
        None => panic!("Team not found.") // TODO: error handling
    };
    render_xml("your-team.xml", vec!(&hunt, &team))
}

#[post("/<hunt_key>/your-team.xml", data="<form>")]
fn post_your_team(hunt_key: String, form: Form<UpdateTeamForm>) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let form = form.into_inner();
    let team = match db.update_team(hunt.id, &form) {
        Ok(team) => team,
        Err(msg) => panic!("{}", msg) // TODO: error handling
    };
    render_xml("your-team.xml", vec!(&hunt, &team))
}


pub fn start() {
    rocket::ignite().mount("/", routes![
        get_css, get_ph, get_js,
        get_index,
        get_signin, post_signin, get_signout, post_signout,
        get_hunt_base, get_hunt,
        get_your_team, post_your_team,
        get_puzzles,
        get_team, get_register,
        post_register,
    ]).launch();
}
