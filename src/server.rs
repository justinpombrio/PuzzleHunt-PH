use std::path::{Path, PathBuf};
use std::fs::File;

use rocket;
use rocket::response::content::Xml;
use rocket::response::Redirect;
use rocket::http::Cookies;

use util::*;
use data::{AddToData, build_data};
use database::Database;
use forms::*;
use cookies::{Puzzler};

// TODO: Every `panic` here should have proper error handling.


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

#[get("/favicon.ico")]
fn get_favicon() -> Option<File> {
    serve_file("favicon.ico")
}


// Global //

#[get("/", rank=0)]
fn get_index() -> Xml<String> {
    let db = Database::new();
    let site = db.get_site();
    let hunts = db.get_hunts();
    render_xml("pages/global/index.xml", vec!(&hunts, &site))
}

#[get("/create-hunt.xml")]
fn get_create_hunt() -> Xml<String> {
    render_xml("pages/global/create-hunt.xml", vec!())
}

#[post("/create-hunt.xml", data="<form>")]
fn post_create_hunt(mut cookies: Cookies, form: CreateHuntForm) -> Redirect {
    let db = Database::new();
    let form = form.into_inner().0;
    match db.create_hunt(&form) {
        Ok(_) => (),
        Err(msg) => panic!("{}", msg)
    };
    if db.signin_admin(&mut cookies, &form.key, &form.password) {
        Redirect::to("/admin/edit-hunt.xml")
    } else {
        panic!("Failed to sign in.")
    }
}


// Admin (not signed in) //

#[get("/admin/signin.xml")]
fn get_admin_signin() -> Xml<String> {
    render_xml("pages/admin/signin.xml", vec!())
}

#[post("/admin/signin.xml", data="<form>")]
fn post_admin_signin(mut cookies: Cookies, form: AdminSignInForm) -> Redirect {
    let db = Database::new();
    let form = form.into_inner();
    if db.signin_admin(&mut cookies, &form.hunt_key, &form.password) {
        Redirect::to("edit-hunt.xml")
    } else {
        panic!("Failed to sign in.")
    }
}


// Admin: Sign Out //

#[get("/admin/signout.xml")]
fn get_admin_signout(mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        None => panic!("Already signed out."),
        Some(hunt) => hunt
    };
    render_xml("pages/admin/signout.xml", vec!(&hunt))
}

#[post("/admin/signout.xml")]
fn post_admin_signout(mut cookies: Cookies) -> Redirect {
    let db = Database::new();
    db.signout_admin(&mut cookies);
    Redirect::to("/")
}


// Admin: Edit Hunt //

#[get("/admin/edit-hunt.xml", rank=1)]
fn get_edit_hunt(mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.")
    };
    println!("Hunt: {:?}", hunt);
    render_xml("pages/admin/edit-hunt.xml", vec!(&hunt))
}

#[post("/admin/edit-hunt.xml", data="<form>")]
fn post_edit_hunt(mut cookies: Cookies, form: EditHuntForm) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.")
    };
    let form = form.into_inner().0;
    let hunt = match db.edit_hunt(&hunt.key, &form) {
        Ok(hunt) => hunt,
        Err(msg) => panic!("{}", msg)
    };
    render_xml("pages/admin/edit-hunt.xml", vec!(&hunt))
}


// Admin: View Teams //

#[get("/admin/view-teams.xml")]
fn get_view_teams(mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.")
    };
    let teams = db.get_all_teams(hunt.id);
    render_xml("pages/admin/view-teams.xml", vec!(&hunt, &teams))
}

#[get("/admin/view-team-email-list.xml")]
fn get_view_team_email_list(mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.")
    };
    let teams = db.get_all_teams(hunt.id);
    render_xml("pages/admin/view-team-email-list.xml", vec!(&hunt, &teams))
}


// Admin: Edit Waves //

#[get("/admin/edit-waves.xml")]
fn get_edit_waves(mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.")
    };
    let waves = db.get_waves(hunt.id);
    render_xml("pages/admin/edit-waves.xml", vec!(&hunt, &waves))
}




// Hunt //

#[get("/<hunt_key>", rank=1)]
fn get_hunt_base(hunt_key: String) -> Redirect {
    Redirect::to(format!("/{}/index.xml", hunt_key))
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
    render_xml("pages/puzzler/team.xml", vec!(&hunt))
}


// Team Page (not signed in) //

#[get("/<hunt_key>/signin.xml", rank=1)]
fn get_signin(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("pages/puzzler/signin.xml", vec!(&hunt))
}

#[post("/<hunt_key>/signin.xml", rank=1, data="<form>")]
fn post_signin(hunt_key: String, mut cookies: Cookies, form: SignInForm) -> Redirect {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let form = form.into_inner();
    if db.signin_team(&mut cookies, hunt.id, &form.name, &form.password) {
        Redirect::to("your-team.xml")
    } else {
        panic!("Failed to sign in.")
    }
}

#[get("/<hunt_key>/register.xml", rank=1)]
fn get_register(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("pages/puzzler/register.xml", vec!(&hunt))
}

#[post("/<hunt_key>/register.xml", data="<form>")]
fn post_register(hunt_key: String, mut cookies: Cookies, form: RegisterForm) -> Redirect {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let form = form.into_inner();
    let team = match db.register(hunt.id, &form.0) {
        Ok(team) => team,
        Err(msg) => panic!("{}", msg)
    };
    db.signin_team(&mut cookies, hunt.id, &team.name, &team.password);
    Redirect::to("your-team.xml")
}


// Team Page (signed in) //

#[get("/<hunt_key>/signout.xml", rank=1)]
fn get_signout(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("pages/puzzler/signout.xml", vec!(&hunt))
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
        None => panic!("Team not found.")
    };
    render_xml("pages/puzzler/your-team.xml", vec!(&hunt, &team))
}

#[post("/<hunt_key>/your-team.xml", data="<form>")]
fn post_your_team(hunt_key: String, form: UpdateTeamForm) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let form = form.into_inner();
    let team = match db.update_team(hunt.id, &form.0) {
        Ok(team) => team,
        Err(msg) => panic!("{}", msg)
    };
    render_xml("pages/puzzler/your-team.xml", vec!(&hunt, &team))
}


// Rocket //

pub fn start() {
    rocket::ignite().mount("/", routes![
        // Resources
        get_css, get_ph, get_js, get_favicon,
        // Site
        get_index,
        get_create_hunt, post_create_hunt,
        get_edit_hunt, post_edit_hunt,
        // Signin
        get_signin, post_signin,
        get_signout, post_signout,
        // Hunt
        get_hunt_base, get_hunt,
        // Team
        get_your_team, post_your_team,
        get_register, post_register,
        get_team, get_team_signedin,
        // Puzzles
        get_puzzles,
        // Admin Signin
        get_admin_signin, post_admin_signin,
        get_admin_signout, post_admin_signout,
        // Admin
        get_view_teams, get_view_team_email_list,
        get_edit_waves
    ]).launch();
}
