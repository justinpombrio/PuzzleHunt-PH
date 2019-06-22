use std::path::{Path, PathBuf};
use std::fs::File;

use rocket;
use rocket::request::Request;
use rocket::response::content::Xml;
use rocket::response::{NamedFile, Redirect};
use rocket::http::Cookies;
use rocket::response::status::NotFound;

use data::Hunt;
use render_page::*;
use database::Database;
use forms::*;
use cookies::{Puzzler};
use expandable_form::{RegularFormResult, ExpandableFormResult};


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


// Shared Functionality //

fn authenticate_admin(cookies: &mut Cookies) -> Result<Hunt, Redirect> {
    let db = Database::new();
    match db.signedin_admin(cookies) {
        Some(hunt) => Ok(hunt),
        None => Err(Redirect::to("/admin/signin.xml"))
    }
}

fn lookup_hunt_OLD(hunt_key: &str) -> Result<Hunt, Redirect> {
    let db = Database::new();
    match db.get_hunt(hunt_key) {
        Some(hunt) => Ok(hunt),
        None => Err(Redirect::to(format!("/hunt-not-found.xml?hunt={}", hunt_key)))
    }
}

fn lookup_hunt(hunt_key: &str) -> Result<Hunt, Page> {
    let db = Database::new();
    match db.get_hunt(hunt_key) {
        Some(hunt) => Ok(hunt),
        None => Err(error("404: Page Not Found",
                          &format!("Hunt '{}' not found. This is not a puzzle.", hunt_key)))
    }
}


// Error Handling //

#[catch(404)]
fn catch_404(req: &Request) -> Xml<String> {
    render_error("404: Page Not Found",
                 &format!("Page '{}' not found. This is not a puzzle.", req.uri()))
}

#[get("/hunt-not-found.xml?<hunt>")]
fn get_hunt_not_found(hunt: String) -> Xml<String> {
    render_error("Hunt Not Found",
                 &format!("There is no hunt called '{}'. This is not a puzzle.", hunt))
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
    let form = match form.into_inner() {
        RegularFormResult::Ok(form) => form,
        RegularFormResult::Err(err) => panic!("{}", err)
    };
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
fn get_admin_signout(mut cookies: Cookies) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        None => return Err(Redirect::to("/")),
        Some(hunt) => hunt
    };
    Ok(render_xml("pages/admin/signout.xml", vec!(&hunt)))
}

#[post("/admin/signout.xml")]
fn post_admin_signout(mut cookies: Cookies) -> Redirect {
    let db = Database::new();
    db.signout_admin(&mut cookies);
    Redirect::to("/")
}


// Admin: Edit Hunt //

#[get("/admin/edit-hunt.xml", rank=1)]
fn get_edit_hunt(mut cookies: Cookies) -> Result<Xml<String>, Redirect> {
    let hunt = authenticate_admin(&mut cookies)?;
    Ok(render_xml("pages/admin/edit-hunt.xml", vec!(&hunt)))
}

#[post("/admin/edit-hunt.xml", data="<form>")]
fn post_edit_hunt(mut cookies: Cookies, form: EditHuntForm) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let form = match form.into_inner() {
        RegularFormResult::Ok(form) => form,
        RegularFormResult::Err(err) =>
            return Ok(render_xml("pages/admin/edit-hunt.xml", vec!(&hunt, &error_msg(&err))))
    };
    match db.edit_hunt(&hunt.key, &form) {
        Ok(hunt) =>
            Ok(render_xml("pages/admin/edit-hunt.xml", vec!(&hunt))),
        Err(msg) =>
            Ok(render_xml("pages/admin/edit-hunt.xml", vec!(&hunt, &error_msg(&msg))))
    }
}


// Admin: View Teams //

#[get("/admin/view-teams.xml")]
fn get_view_teams(mut cookies: Cookies) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let teams = db.get_teams(hunt.id);
    Ok(render_xml("pages/admin/view-teams.xml", vec!(&hunt, &teams)))
}

#[get("/admin/view-team-email-list.xml")]
fn get_view_team_email_list(mut cookies: Cookies) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let teams = db.get_teams(hunt.id);
    Ok(render_xml("pages/admin/view-team-email-list.xml", vec!(&hunt, &teams)))
}


// Admin: Edit Waves //

#[get("/admin/edit-waves.xml")]
fn get_edit_waves(mut cookies: Cookies) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let waves = db.get_waves(hunt.id);
    Ok(render_xml("pages/admin/edit-waves.xml", vec!(&hunt, &waves)))
}

#[post("/admin/edit-waves.xml", data="<form>")]
fn post_edit_waves(mut cookies: Cookies, form: WavesForm) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let waves = match form.into_inner() {
        ExpandableFormResult::Ok(form) => form.waves,
        ExpandableFormResult::Err(err) => panic!("{}", err)
    };
    db.set_waves(hunt.id, &waves);
    Ok(render_xml("pages/admin/edit-waves.xml", vec!(&hunt, &waves)))
}


// Admin: Edit Puzzles //

#[get("/admin/edit-puzzles.xml")]
fn get_edit_puzzles(mut cookies: Cookies) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let puzzles = db.get_puzzles(hunt.id);
    Ok(render_xml("pages/admin/edit-puzzles.xml", vec!(&hunt, &puzzles)))
}

#[post("/admin/edit-puzzles.xml", data="<form>")]
fn post_edit_puzzles(mut cookies: Cookies, form: PuzzlesForm) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let puzzles = match form.into_inner() {
        ExpandableFormResult::Ok(form) => form.puzzles,
        ExpandableFormResult::Err(err) => panic!("{}", err)
    };
    db.set_puzzles(hunt.id, &puzzles);
    Ok(render_xml("pages/admin/edit-puzzles.xml", vec!(&hunt, &puzzles)))
}

#[get("/admin/edit-hints.xml")]
fn get_edit_hints(mut cookies: Cookies) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let hints = db.get_hints(hunt.id);
    Ok(render_xml("pages/admin/edit-hints.xml", vec!(&hunt, &hints)))
}

#[post("/admin/edit-hints.xml", data="<form>")]
fn post_edit_hints(mut cookies: Cookies, form: HintsForm) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let hints = match form.into_inner() {
        ExpandableFormResult::Ok(form) => form.hints,
        ExpandableFormResult::Err(err) => panic!("{}", err)
    };
    db.set_hints(hunt.id, &hints);
    Ok(render_xml("pages/admin/edit-hints.xml", vec!(&hunt, &hints)))
}


// Hunt //

#[get("/<hunt_key>", rank=1)]
fn get_hunt_base(hunt_key: String) -> Redirect {
    Redirect::to(format!("/{}/index.xml", hunt_key))
}

#[get("/<hunt_key>/index.xml", rank=1)]
fn get_hunt(hunt_key: String) -> Result<Xml<String>, Redirect> {
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    Ok(render_xml(format!("hunts/{}/index.xml", hunt.key), vec!(&hunt)))
}


// Puzzles //

#[get("/<hunt_key>/puzzles.xml", rank=1)]
fn get_puzzles(mut cookies: Cookies, hunt_key: String) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = lookup_hunt(&hunt_key)?;
    let team = db.signedin_team(&mut cookies);
    let waves: Vec<_> = db.get_released_waves(hunt.id, &team);
    Ok(xml("pages/puzzler/puzzles.xml", vec!(&hunt, &waves)))
}

#[get("/<hunt_key>/puzzle/<puzzle_key>", rank = 1)]
fn get_puzzle(hunt_key: String, puzzle_key: String) -> Result<NamedFile, NotFound<String>> {
    let path = format!("hunts/{}/puzzle/{}", hunt_key, puzzle_key);
    NamedFile::open(&Path::new(&path)).map_err(|_| NotFound("Puzzle not found.".to_string()))
}

#[get("/<hunt_key>/hint/<hint_key>", rank = 1)]
fn get_hint(hunt_key: String, hint_key: String) -> Result<Xml<String>, Redirect> {
    if !hint_key.ends_with(".xml") {
        panic!("Hint not found!");
    }
    let hint_key = &hint_key[0 .. hint_key.len()-4];
    let db = Database::new();
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    let hint = db.get_released_hint(hunt.id, &hint_key)
        .expect("Hint not found!");
    Ok(render_xml("pages/puzzler/hint.xml", vec!(&hunt, &hint)))
}


// Submitting Answers //

#[get("/<hunt_key>/submit-answer/<puzzle_key>", rank=1)]
fn get_submit_answer(mut cookies: Cookies, hunt_key: String, puzzle_key: String) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    let team = match db.signedin_team(&mut cookies) {
        Some(team) => team,
        None => panic!("Team not found.")
    };
    let puzzle = match db.get_released_puzzle(hunt.id, &puzzle_key) {
        None => panic!("Puzzle not found (or not yet released) {}", puzzle_key),
        Some(p) => p
    };
    // If they're out of guesses, give them the bad news and don't show the form.
    if let Some(judgement) = db.out_of_guesses(&team) {
        return Ok(render_xml("pages/puzzler/submit-answer.xml", vec!(&hunt, &team, &puzzle, &judgement)));
    }
    // Otherwise show the regular form.
    Ok(render_xml("pages/puzzler/submit-answer.xml", vec!(&hunt, &team, &puzzle)))
}

#[post("/<hunt_key>/submit-answer/<puzzle_key>", data="<form>", rank=1)]
fn post_submit_answer(
    mut cookies: Cookies, hunt_key: String, puzzle_key: String, form: SubmitAnswerForm)
    -> Result<Xml<String>, Redirect>
{
    let db = Database::new();
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    let team = match db.signedin_team(&mut cookies) {
        Some(team) => team,
        None => panic!("Team not found.")
    };
    let puzzle = match db.get_released_puzzle(hunt.id, &puzzle_key) {
        None => panic!("Puzzle not found (or not yet released) {}", puzzle_key),
        Some(p) => p
    };
    let form = match form.into_inner() {
        RegularFormResult::Ok(form) => form,
        RegularFormResult::Err(err) => panic!("{}", err)
    };
    let judgement = db.submit_guess(&team, &puzzle, &form.guess);
    // Update team in case the guesses were decremented
    let team = match db.signedin_team(&mut cookies) {
        Some(team) => team,
        None => panic!("Team not found.")
    };
    Ok(render_xml("pages/puzzler/submit-answer.xml", vec!(&hunt, &team, &puzzle, &judgement)))
}


// Team Page //

#[get("/<hunt_key>/team.xml", rank=1)]
#[allow(unused_variables)]
fn get_team_signedin(hunt_key: String, puzzler: Puzzler) -> Redirect {
    Redirect::to("your-team.xml")
}

#[get("/<hunt_key>/team.xml", rank=2)]
fn get_team(hunt_key: String) -> Result<Xml<String>, Redirect> {
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    Ok(render_xml("pages/puzzler/team.xml", vec!(&hunt)))
}


// Team Page (not signed in) //

#[get("/<hunt_key>/signin.xml", rank=1)]
fn get_signin(hunt_key: String) -> Result<Xml<String>, Redirect> {
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    Ok(render_xml("pages/puzzler/signin.xml", vec!(&hunt)))
}

#[post("/<hunt_key>/signin.xml", rank=1, data="<form>")]
fn post_signin(hunt_key: String, mut cookies: Cookies, form: SignInForm) -> Result<Redirect, Redirect> {
    let db = Database::new();
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    let form = form.into_inner();
    if db.signin_team(&mut cookies, hunt.id, &form.name, &form.password) {
        Ok(Redirect::to("your-team.xml"))
    } else {
        panic!("Failed to sign in.")
    }
}

#[get("/<hunt_key>/register.xml", rank=1)]
fn get_register(hunt_key: String) -> Result<Xml<String>, Redirect> {
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    Ok(render_xml("pages/puzzler/register.xml", vec!(&hunt)))
}

#[post("/<hunt_key>/register.xml", data="<form>")]
fn post_register(hunt_key: String, mut cookies: Cookies, form: CreateTeamForm) -> Result<Redirect, Redirect> {
    let db = Database::new();
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    let form = match form.into_inner() {
        ExpandableFormResult::Ok(form) => form,
        ExpandableFormResult::Err(err) => panic!("{}", err)
    };
    let team = match db.create_team(hunt.id, &form) {
        Ok(team) => team,
        Err(msg) => panic!("{}", msg)
    };
    db.signin_team(&mut cookies, hunt.id, &team.name, &team.password);
    Ok(Redirect::to("your-team.xml"))
}


// Team Page (signed in) //

#[get("/<hunt_key>/signout.xml", rank=1)]
fn get_signout(hunt_key: String) -> Result<Xml<String>, Redirect> {
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    Ok(render_xml("pages/puzzler/signout.xml", vec!(&hunt)))
}

#[post("/<hunt_key>/signout.xml")]
#[allow(unused_variables)]
fn post_signout(hunt_key: String, mut cookies: Cookies) -> Redirect {
    let db = Database::new();
    db.signout_team(&mut cookies);
    Redirect::to(".")
}

#[get("/<hunt_key>/your-team.xml", rank=1)]
fn get_your_team(hunt_key: String, mut cookies: Cookies) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    let team = match db.signedin_team(&mut cookies) {
        Some(team) => team,
        None => panic!("Team not found.")
    };
    Ok(render_xml("pages/puzzler/your-team.xml", vec!(&hunt, &team)))
}

#[post("/<hunt_key>/your-team.xml", data="<form>")]
fn post_your_team(hunt_key: String, form: UpdateTeamForm) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    let form = match form.into_inner() {
        ExpandableFormResult::Ok(form) => form,
        ExpandableFormResult::Err(err) => panic!("{}", err)
    };
    let team = match db.update_team(hunt.id, &form) {
        Ok(team) => team,
        Err(msg) => panic!("{}", msg)
    };
    Ok(render_xml("pages/puzzler/your-team.xml", vec!(&hunt, &team)))
}

// Puzzle Stats

#[get("/<hunt_key>/puzzle-stats.xml", rank=1)]
fn get_puzzle_stats(hunt_key: String) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    let stats: Vec<_> = db.get_puzzle_stats(hunt.id);
    Ok(render_xml("pages/puzzler/puzzle-stats.xml", vec!(&hunt, &stats)))
}

#[get("/<hunt_key>/leaderboard.xml", rank=1)]
fn get_team_stats(hunt_key: String) -> Result<Xml<String>, Redirect> {
    let db = Database::new();
    let hunt = lookup_hunt_OLD(&hunt_key)?;
    let teams: Vec<_> = db.get_team_stats(hunt.id);
    Ok(render_xml("pages/puzzler/leaderboard.xml", vec!(&hunt, &teams)))
}


// Rocket //

pub fn start() {
    rocket::ignite().mount("/", routes![
        // Resources
        get_css, get_ph, get_js, get_favicon,
        // Errors
        get_hunt_not_found,
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
        get_puzzles, get_puzzle, get_hint,
        get_submit_answer, post_submit_answer,
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
    ])
        .register(catchers![catch_404])
        .launch();
}
