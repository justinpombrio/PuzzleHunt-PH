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
use forms::{CreateHuntForm, SignInForm, RegisterForm, UpdateTeamForm};
use cookies::{Puzzler};


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


// Site //

#[get("/", rank=0)]
fn get_index() -> Xml<String> {
    let db = Database::new();
    let site = db.get_site();
    let hunts = db.get_hunts();
    render_xml("admin-pages/index.xml", vec!(&hunts, &site))
}

#[get("/create-hunt.xml")]
fn get_create_hunt() -> Xml<String> {
    render_xml("admin-pages/create-hunt.xml", vec!())
}

#[post("/create-hunt.xml", data="<form>")]
fn post_create_hunt(form: Form<CreateHuntForm>) -> Redirect {
    let db = Database::new();
    let form = form.into_inner();
    let hunt = match db.create_hunt(&form) {
        Ok(team) => team,
        Err(msg) => panic!("{}", msg) // TODO: error handling
    };
    Redirect::to("/admin/edit-hunt.xml")
}


// Admin: Update Hunt //

#[get("/admin/edit-hunt.xml", rank=1)]
fn get_edit_hunt(mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.") // TODO: error handling
    };
    render_xml("admin-pages/edit-hunt.xml", vec!(&hunt))
}


// Hunt //

#[get("/<hunt_key>", rank=1)]
fn get_hunt_base(hunt_key: String) -> Redirect {
    Redirect::to(&format!("/{}/index.xml", hunt_key))
}

#[get("/<hunt_key>/index.xml", rank=1)]
fn get_hunt(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml(format!("hunts/{}/index.xml", hunt.key), vec!(&hunt))
}

#[get("/<hunt_key>/puzzles.xml", rank=1)]
fn get_puzzles(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let waves = db.get_waves(hunt.id);
    render_xml(format!("hunts/{}/puzzles.xml", hunt.key), vec!(&hunt, &waves))
}


// Team Page //

#[get("/<hunt_key>/team.xml", rank=1)]
#[allow(unused_variables)]
fn get_team_signedin(hunt_key: String, puzzler: Puzzler) -> Redirect {
    Redirect::to("your-team.xml")
}

#[get("/<hunt_key>/team.xml", rank=2)]
fn get_team(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("pages/team.xml", vec!(&hunt))
}


// Team Page (not signed in) //

#[get("/<hunt_key>/signin.xml", rank=1)]
fn get_signin(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("pages/signin.xml", vec!(&hunt))
}

#[post("/<hunt_key>/signin.xml", rank=1, data="<form>")]
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

#[get("/<hunt_key>/register.xml", rank=1)]
fn get_register(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("pages/register.xml", vec!(&hunt))
}

#[post("/<hunt_key>/register.xml", data="<form>")]
fn post_register(hunt_key: String, mut cookies: Cookies, form: Form<RegisterForm>) -> Redirect {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let form = form.into_inner();
    let team = match db.register(hunt.id, &form) {
        Ok(team) => team,
        Err(msg) => panic!("{}", msg) // TODO: error handling
    };
    db.signin_team(&mut cookies, hunt.id, &team.name, &team.password);
    Redirect::to("your-team.xml")
}


// Team Page (signed in) //

#[get("/<hunt_key>/signout.xml", rank=1)]
fn get_signout(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("pages/signout.xml", vec!(&hunt))
}

#[post("/<hunt_key>/signout.xml")]
#[allow(unused_variables)]
fn post_signout(hunt_key: String, mut cookies: Cookies) -> Redirect {
    let db = Database::new();
    db.signout_team(&mut cookies);
    Redirect::to(".")
}

#[get("/<hunt_key>/your-team.xml", rank=1)]
fn get_your_team(hunt_key: String, mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let team = match db.signedin_team(&mut cookies) {
        Some(team) => team,
        None => panic!("Team not found.") // TODO: error handling
    };
    render_xml("pages/your-team.xml", vec!(&hunt, &team))
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
    render_xml("pages/your-team.xml", vec!(&hunt, &team))
}


// Rocket //

pub fn start() {
    rocket::ignite().mount("/", routes![
        // Resources
        get_css, get_ph, get_js,
        // Site
        get_index,
        get_create_hunt, post_create_hunt,
        // Signin
        get_signin, post_signin, get_signout, post_signout,
        // Hunt
        get_hunt_base, get_hunt,
        // Team
        get_your_team, post_your_team,
        get_register, post_register,
        // Puzzles
        get_puzzles,
        // Admin
        get_team, get_team_signedin
    ]).launch();
}
