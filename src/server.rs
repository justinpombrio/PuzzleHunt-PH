use std::path::{Path, PathBuf};
use std::fs::File;

use rocket;
use rocket::response::content::Xml;
use rocket::response::{NamedFile, Redirect};
use rocket::http::Cookies;
use rocket::response::status::NotFound;

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


// Site //

#[get("/", rank=0)]
fn get_index() -> Xml<String> {
    let db = Database::new();
    let site = db.get_site();
    let hunts = db.get_hunts();
    render_xml("pages/site/index.xml", vec!(&hunts, &site))
}

#[get("/create-hunt.xml")]
fn get_create_hunt() -> Xml<String> {
    render_xml("pages/site/create-hunt.xml", vec!())
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
        panic!("Failed to sign in as admin.")
    }
}


// Admin (not signed in) //

#[get("/admin")]
fn get_admin() -> Redirect {
    Redirect::to("/admin/signin.xml")
}

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
        panic!("Failed to sign in as admin.")
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
    let teams = db.get_teams(hunt.id);
    render_xml("pages/admin/view-teams.xml", vec!(&hunt, &teams))
}

#[get("/admin/view-team-email-list.xml")]
fn get_view_team_email_list(mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.")
    };
    let teams = db.get_teams(hunt.id);
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

#[post("/admin/edit-waves.xml", data="<form>")]
fn post_edit_waves(mut cookies: Cookies, form: WavesForm) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.")
    };
    let waves = form.into_inner().0.waves;
    db.set_waves(hunt.id, &waves);
    render_xml("pages/admin/edit-waves.xml", vec!(&hunt, &waves))
}

// Admin: Edit Puzzles //

#[get("/admin/edit-puzzles.xml")]
fn get_edit_puzzles(mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.")
    };
    let puzzles = db.get_puzzles(hunt.id);
    render_xml("pages/admin/edit-puzzles.xml", vec!(&hunt, &puzzles))
}

#[post("/admin/edit-puzzles.xml", data="<form>")]
fn post_edit_puzzles(mut cookies: Cookies, form: PuzzlesForm) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.")
    };
    let puzzles = form.into_inner().0.puzzles;
    db.set_puzzles(hunt.id, &puzzles);
    render_xml("pages/admin/edit-puzzles.xml", vec!(&hunt, &puzzles))
}

#[get("/admin/edit-hints.xml")]
fn get_edit_hints(mut cookies: Cookies) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found")
    };
    let hints = db.get_hints(hunt.id);
    render_xml("pages/admin/edit-hints.xml", vec!(&hunt, &hints))
}

#[post("/admin/edit-hints.xml", data="<form>")]
fn post_edit_hints(mut cookies: Cookies, form: HintsForm) -> Xml<String> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        Some(hunt) => hunt,
        None => panic!("Hunt not found.")
    };
    let hints = form.into_inner().0.hints;
    db.set_hints(hunt.id, &hints);
    render_xml("pages/admin/edit-hints.xml", vec!(&hunt, &hints))
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


// Puzzles //

#[get("/<hunt_key>/puzzles.xml", rank=1)]
fn get_puzzles(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let waves: Vec<_> = db.get_released_waves(hunt.id)
        .into_iter()
        .filter(|w| w.puzzles.len() > 0)
        .collect();
    render_xml("pages/puzzler/puzzles.xml", vec!(&hunt, &waves))
}

#[get("/<hunt_key>/puzzle/<puzzle_name>", rank = 1)]
fn get_puzzle(hunt_key: String, puzzle_name: String) -> Result<NamedFile, NotFound<String>> {
    let path = format!("hunts/{}/puzzle/{}", hunt_key, puzzle_name);
    NamedFile::open(&Path::new(&path)).map_err(|_| NotFound("Puzzle not found.".to_string()))
}

#[get("/<hunt_key>/hint/<hint_key>", rank = 1)]
fn get_hint(hunt_key: String, hint_key: String) -> Xml<String> {
    if !hint_key.ends_with(".xml") {
        panic!("Hint not found!");
    }
    let hint_key = &hint_key[0 .. hint_key.len()-4];
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let hint = db.get_released_hint(hunt.id, &hint_key)
        .expect("Hint not found!");
    render_xml("pages/puzzler/hint.xml", vec!(&hunt, &hint))
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
fn post_register(hunt_key: String, mut cookies: Cookies, form: CreateTeamForm) -> Redirect {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let form = form.into_inner().0;
    let team = match db.create_team(hunt.id, &form) {
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
    let form = form.into_inner().0;
    let team = match db.update_team(hunt.id, &form) {
        Ok(team) => team,
        Err(msg) => panic!("{}", msg)
    };
    render_xml("pages/puzzler/your-team.xml", vec!(&hunt, &team))
}

// Puzzle Stats

#[get("/<hunt_key>/puzzle-stats.xml", rank=1)]
fn get_puzzle_stats(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let stats: Vec<_> = db.get_puzzle_stats(hunt.id);
    render_xml("pages/puzzler/puzzle-stats.xml", vec!(&hunt, &stats))
}

#[get("/<hunt_key>/leaderboard.xml", rank=1)]
fn get_team_stats(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let teams: Vec<_> = db.get_team_stats(hunt.id);
    render_xml("pages/puzzler/leaderboard.xml", vec!(&hunt, &teams))
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
        get_puzzles, get_puzzle,
        get_hint,
        // Stats
        get_puzzle_stats, get_team_stats,
        // Admin Signin
        get_admin, get_admin_signin, post_admin_signin,
        get_admin_signout, post_admin_signout,
        // Admin
        get_view_teams, get_view_team_email_list,
        get_edit_waves, post_edit_waves,
        get_edit_puzzles, post_edit_puzzles,
        get_edit_hints, post_edit_hints,
    ]).launch();
}
