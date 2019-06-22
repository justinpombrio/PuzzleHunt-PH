use std::path::{Path, PathBuf};

use rocket;
use rocket::request::Request;
use rocket::http::Cookies;

use data::{Hunt, Team, ReleasedPuzzle};
use page::{Page, file, xml, redirect, error, not_found, error_msg};
use database::Database;
use forms::*;
use cookies::{Puzzler};
use expandable_form::{RegularFormResult, ExpandableFormResult};


// Resources //

#[get("/css/<path..>")]
fn get_css(path: PathBuf) -> Page {
    file(Path::new("css/").join(path))
}

#[get("/ph.xsl")]
fn get_ph() -> Page {
    file("ph.xsl")
}

#[get("/ph.js")]
fn get_js() -> Page {
    file("ph.js")
}

#[get("/favicon.ico")]
fn get_favicon() -> Page {
    file("favicon.ico")
}


// Shared Functionality //

fn authenticate_admin(cookies: &mut Cookies) -> Result<Hunt, Page> {
    let db = Database::new();
    match db.signedin_admin(cookies) {
        Some(hunt) => Ok(hunt),
        None => Err(redirect("/admin/signin.xml"))
    }
}

fn authenticate_team(hunt_key: &str, cookies: &mut Cookies) -> Result<Team, Page> {
    let db = Database::new();
    match db.signedin_team(cookies) {
        Some(team) => Ok(team),
        None => Err(redirect(format!("/{}/signin.xml", hunt_key)))
    }
}

fn lookup_hunt(hunt_key: &str) -> Result<Hunt, Page> {
    let db = Database::new();
    match db.get_hunt(hunt_key) {
        Some(hunt) => Ok(hunt),
        None => Err(not_found(&format!("Hunt '{}' not found. This is not a puzzle.", hunt_key)))
    }
}

fn lookup_puzzle(hunt: &Hunt, puzzle_key: &str) -> Result<ReleasedPuzzle, Page> {
    let db = Database::new();
    match db.get_released_puzzle(hunt.id, &puzzle_key) {
        Some(p) => Ok(p),
        None => Err(error("Puzzle Not Found", "Puzzle not found (or not yet released)."))
    }
}


// Error Handling //

#[catch(404)]
fn catch_404(req: &Request) -> Page {
    error("404: Page Not Found",
          &format!("Page '{}' not found. This is not a puzzle.", req.uri()))
}


// Site //

#[get("/", rank=0)]
fn get_index() -> Page {
    let db = Database::new();
    let site = db.get_site();
    let hunts = db.get_hunts();
    xml("pages/site/index.xml", vec!(&hunts, &site))
}

#[get("/create-hunt.xml")]
fn get_create_hunt() -> Page {
    xml("pages/site/create-hunt.xml", vec!())
}

#[post("/create-hunt.xml", data="<form>")]
fn post_create_hunt(mut cookies: Cookies, form: CreateHuntForm) -> Page {
    let db = Database::new();
    let form = match form.into_inner() {
        RegularFormResult::Ok(form) => form,
        RegularFormResult::Err(err) =>
            return xml("pages/site/create-hunt.xml", vec!(&error_msg(&err)))
    };
    match db.create_hunt(&form) {
        Ok(_) => (),
        Err(msg) => return error("Failed to Create Hunt", &msg)
    };
    if db.signin_admin(&mut cookies, &form.key, &form.password) {
        redirect("/admin/edit-hunt.xml")
    } else {
        error("Internal Error", "Failed to sign in as admin after hunt creation.")
    }
}


// Admin (not signed in) //

#[get("/admin")]
fn get_admin() -> Page {
    redirect("/admin/signin.xml")
}

#[get("/admin/signin.xml")]
fn get_admin_signin() -> Page {
    xml("pages/admin/signin.xml", vec!())
}

#[post("/admin/signin.xml", data="<form>")]
fn post_admin_signin(mut cookies: Cookies, form: AdminSignInForm) -> Page {
    let db = Database::new();
    let form = form.into_inner();
    if db.signin_admin(&mut cookies, &form.hunt_key, &form.password) {
        redirect("edit-hunt.xml")
    } else {
        xml("pages/admin/signin.xml",
            vec!(&error_msg("Failed to sign in as admin."))
    }
}


// Admin: Sign Out //

#[get("/admin/signout.xml")]
fn get_admin_signout(mut cookies: Cookies) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = match db.signedin_admin(&mut cookies) {
        None => return Err(redirect("/")),
        Some(hunt) => hunt
    };
    Ok(xml("pages/admin/signout.xml", vec!(&hunt)))
}

#[post("/admin/signout.xml")]
fn post_admin_signout(mut cookies: Cookies) -> Page {
    let db = Database::new();
    db.signout_admin(&mut cookies);
    redirect("/")
}


// Admin: Edit Hunt //

#[get("/admin/edit-hunt.xml", rank=1)]
fn get_edit_hunt(mut cookies: Cookies) -> Result<Page, Page> {
    let hunt = authenticate_admin(&mut cookies)?;
    Ok(xml("pages/admin/edit-hunt.xml", vec!(&hunt)))
}

#[post("/admin/edit-hunt.xml", data="<form>")]
fn post_edit_hunt(mut cookies: Cookies, form: EditHuntForm) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let form = match form.into_inner() {
        RegularFormResult::Ok(form) => form,
        RegularFormResult::Err(err) =>
            return Ok(xml("pages/admin/edit-hunt.xml", vec!(&hunt, &error_msg(&err))))
    };
    match db.edit_hunt(&hunt.key, &form) {
        Ok(hunt) =>
            Ok(xml("pages/admin/edit-hunt.xml", vec!(&hunt))),
        Err(msg) =>
            Ok(xml("pages/admin/edit-hunt.xml", vec!(&hunt, &error_msg(&msg))))
    }
}


// Admin: View Teams //

#[get("/admin/view-teams.xml")]
fn get_view_teams(mut cookies: Cookies) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let teams = db.get_teams(hunt.id);
    Ok(xml("pages/admin/view-teams.xml", vec!(&hunt, &teams)))
}

#[get("/admin/view-team-email-list.xml")]
fn get_view_team_email_list(mut cookies: Cookies) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let teams = db.get_teams(hunt.id);
    Ok(xml("pages/admin/view-team-email-list.xml", vec!(&hunt, &teams)))
}


// Admin: Edit Waves //

#[get("/admin/edit-waves.xml")]
fn get_edit_waves(mut cookies: Cookies) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let waves = db.get_waves(hunt.id);
    Ok(xml("pages/admin/edit-waves.xml", vec!(&hunt, &waves)))
}

#[post("/admin/edit-waves.xml", data="<form>")]
fn post_edit_waves(mut cookies: Cookies, form: WavesForm) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let waves = match form.into_inner() {
        ExpandableFormResult::Ok(form) => form.waves,
        ExpandableFormResult::Err(err) => {
            let old_waves = db.get_waves(hunt.id);
            return Err(xml("pages/admin/edit-waves.xml",
                    vec!(&hunt, &old_waves, &error_msg(&err))));
        }
    };
    db.set_waves(hunt.id, &waves);
    Ok(xml("pages/admin/edit-waves.xml", vec!(&hunt, &waves)))
}


// Admin: Edit Puzzles //

#[get("/admin/edit-puzzles.xml")]
fn get_edit_puzzles(mut cookies: Cookies) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let puzzles = db.get_puzzles(hunt.id);
    Ok(xml("pages/admin/edit-puzzles.xml", vec!(&hunt, &puzzles)))
}

#[post("/admin/edit-puzzles.xml", data="<form>")]
fn post_edit_puzzles(mut cookies: Cookies, form: PuzzlesForm) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let puzzles = match form.into_inner() {
        ExpandableFormResult::Ok(form) => form.puzzles,
        ExpandableFormResult::Err(err) => {
            let old_puzzles = db.get_puzzles(hunt.id);
            return Err(xml("pages/admin/edit-puzzles.xml",
                           vec!(&hunt, &old_puzzles, &error_msg(&err))))
        }
    };
    db.set_puzzles(hunt.id, &puzzles);
    let puzzles = db.get_puzzles(hunt.id);
    Ok(xml("pages/admin/edit-puzzles.xml", vec!(&hunt, &puzzles)))
}

#[get("/admin/edit-hints.xml")]
fn get_edit_hints(mut cookies: Cookies) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let hints = db.get_hints(hunt.id);
    Ok(xml("pages/admin/edit-hints.xml", vec!(&hunt, &hints)))
}

#[post("/admin/edit-hints.xml", data="<form>")]
fn post_edit_hints(mut cookies: Cookies, form: HintsForm) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = authenticate_admin(&mut cookies)?;
    let hints = match form.into_inner() {
        ExpandableFormResult::Ok(form) => form.hints,
        ExpandableFormResult::Err(err) => {
            let old_hints = db.get_hints(hunt.id);
            return Err(xml("pages/admin/edit-hints.xml",
                           vec!(&hunt, &old_hints, &error_msg(&err))))
        }
    };
    db.set_hints(hunt.id, &hints);
    let hints = db.get_hints(hunt.id);
    Ok(xml("pages/admin/edit-hints.xml", vec!(&hunt, &hints)))
}


// Hunt //

#[get("/<hunt_key>", rank=1)]
fn get_hunt_base(hunt_key: String) -> Page {
    redirect(format!("/{}/index.xml", hunt_key))
}

#[get("/<hunt_key>/index.xml", rank=1)]
fn get_hunt(hunt_key: String) -> Result<Page, Page> {
    let hunt = lookup_hunt(&hunt_key)?;
    Ok(xml(format!("hunts/{}/index.xml", hunt.key), vec!(&hunt)))
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
fn get_puzzle(hunt_key: String, puzzle_key: String) -> Page {
    let path = format!("hunts/{}/puzzle/{}", hunt_key, puzzle_key);
    file(&Path::new(&path))
}

#[get("/<hunt_key>/hint/<hint_key>", rank = 1)]
fn get_hint(hunt_key: String, hint_key: String) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = lookup_hunt(&hunt_key)?;
    let hint = match db.get_released_hint(hunt.id, &hint_key) {
        Some(hint) => hint,
        None => return Err(not_found(&format!("Hint '{}' not found. This is not a puzzle.", hunt_key)))
    };
    Ok(xml("pages/puzzler/hint.xml", vec!(&hunt, &hint)))
}


// Submitting Answers //

#[get("/<hunt_key>/submit-answer/<puzzle_key>", rank=1)]
fn get_submit_answer(mut cookies: Cookies, hunt_key: String, puzzle_key: String) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = lookup_hunt(&hunt_key)?;
    let team = authenticate_team(&hunt_key, &mut cookies)?;
    let puzzle = lookup_puzzle(&hunt, &puzzle_key)?;
    // If they're out of guesses, give them the bad news and don't show the form.
    if let Some(judgement) = db.out_of_guesses(&team) {
        return Ok(xml("pages/puzzler/submit-answer.xml", vec!(&hunt, &team, &puzzle, &judgement)));
    }
    // Otherwise show the regular form.
    Ok(xml("pages/puzzler/submit-answer.xml", vec!(&hunt, &team, &puzzle)))
}

#[post("/<hunt_key>/submit-answer/<puzzle_key>", data="<form>", rank=1)]
fn post_submit_answer(
    mut cookies: Cookies, hunt_key: String, puzzle_key: String, form: SubmitAnswerForm)
    -> Result<Page, Page>
{
    let db = Database::new();
    let hunt = lookup_hunt(&hunt_key)?;
    let team = authenticate_team(&hunt_key, &mut cookies)?;
    let puzzle = lookup_puzzle(&hunt, &puzzle_key)?;
    let form = match form.into_inner() {
        RegularFormResult::Ok(form) => form,
        RegularFormResult::Err(err) => {
            return Err(xml("pages/puzzler/submit-answer.xml",
                           vec!(&hunt, &team, &puzzle, &error_msg(&err))))
        }
    };
    let judgement = db.submit_guess(&team, &puzzle, &form.guess);
    // Update team in case the guesses were decremented
    let team = authenticate_team(&hunt_key, &mut cookies)?;
    Ok(xml("pages/puzzler/submit-answer.xml", vec!(&hunt, &team, &puzzle, &judgement)))
}


// Team Page //

#[get("/<hunt_key>/team.xml", rank=1)]
#[allow(unused_variables)]
fn get_team_signedin(hunt_key: String, puzzler: Puzzler) -> Page {
    redirect("your-team.xml")
}

#[get("/<hunt_key>/team.xml", rank=2)]
fn get_team(hunt_key: String) -> Result<Page, Page> {
    let hunt = lookup_hunt(&hunt_key)?;
    Ok(xml("pages/puzzler/team.xml", vec!(&hunt)))
}


// Team Page (not signed in) //

#[get("/<hunt_key>/signin.xml", rank=1)]
fn get_signin(hunt_key: String) -> Result<Page, Page> {
    let hunt = lookup_hunt(&hunt_key)?;
    Ok(xml("pages/puzzler/signin.xml", vec!(&hunt)))
}

#[post("/<hunt_key>/signin.xml", rank=1, data="<form>")]
fn post_signin(hunt_key: String, mut cookies: Cookies, form: SignInForm) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = lookup_hunt(&hunt_key)?;
    let form = form.into_inner();
    if db.signin_team(&mut cookies, hunt.id, &form.name, &form.password) {
        Ok(redirect(format!("/{}/your-team.xml", hunt_key)))
    } else {
        Err(xml(&format!("pages/puzzler/signin.xml"),
                vec!(&hunt, &error_msg("Failed to sign in."))))
    }
}

#[get("/<hunt_key>/register.xml", rank=1)]
fn get_register(hunt_key: String) -> Result<Page, Page> {
    let hunt = lookup_hunt(&hunt_key)?;
    Ok(xml("pages/puzzler/register.xml", vec!(&hunt)))
}

#[post("/<hunt_key>/register.xml", data="<form>")]
fn post_register(hunt_key: String, mut cookies: Cookies, form: CreateTeamForm) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = lookup_hunt(&hunt_key)?;
    let form = match form.into_inner() {
        ExpandableFormResult::Ok(form) => form,
        ExpandableFormResult::Err(err) => {
            return Err(xml("pages/puzzler/register.xml",
                           vec!(&hunt, &error_msg(&err))))
        }
    };
    let team = match db.create_team(hunt.id, &form) {
        Ok(team) => team,
        Err(err) => {
            return Err(xml("pages/puzzler/register.xml",
                           vec!(&hunt, &error_msg(&err))))
        }
    };
    db.signin_team(&mut cookies, hunt.id, &team.name, &team.password);
    Ok(redirect("your-team.xml"))
}


// Team Page (signed in) //

#[get("/<hunt_key>/signout.xml", rank=1)]
fn get_signout(hunt_key: String) -> Result<Page, Page> {
    let hunt = lookup_hunt(&hunt_key)?;
    Ok(xml("pages/puzzler/signout.xml", vec!(&hunt)))
}

#[post("/<hunt_key>/signout.xml")]
#[allow(unused_variables)]
fn post_signout(hunt_key: String, mut cookies: Cookies) -> Page {
    let db = Database::new();
    db.signout_team(&mut cookies);
    redirect(".")
}

#[get("/<hunt_key>/your-team.xml", rank=1)]
fn get_your_team(hunt_key: String, mut cookies: Cookies) -> Result<Page, Page> {
    let hunt = lookup_hunt(&hunt_key)?;
    let team = authenticate_team(&hunt_key, &mut cookies)?;
    Ok(xml("pages/puzzler/your-team.xml", vec!(&hunt, &team)))
}

#[post("/<hunt_key>/your-team.xml", data="<form>")]
fn post_your_team(hunt_key: String, form: UpdateTeamForm, mut cookies: Cookies) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = lookup_hunt(&hunt_key)?;
    let team = authenticate_team(&hunt_key, &mut cookies)?;
    let form = match form.into_inner() {
        ExpandableFormResult::Ok(form) => form,
        ExpandableFormResult::Err(err) => {
            return Err(xml("pages/puzzler/your-team.xml",
                           vec!(&hunt, &team, &error_msg(&err))))
        }
    };
    let team = match db.update_team(hunt.id, &form) {
        Ok(team) => team,
        Err(err) => {
            return Err(xml("pages/puzzler/your-team.xml",
                           vec!(&hunt, &team, &error_msg(&err))))
        }
    };
    Ok(xml("pages/puzzler/your-team.xml", vec!(&hunt, &team)))
}

// Puzzle Stats

#[get("/<hunt_key>/puzzle-stats.xml", rank=1)]
fn get_puzzle_stats(hunt_key: String) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = lookup_hunt(&hunt_key)?;
    let stats: Vec<_> = db.get_puzzle_stats(hunt.id);
    Ok(xml("pages/puzzler/puzzle-stats.xml", vec!(&hunt, &stats)))
}

#[get("/<hunt_key>/leaderboard.xml", rank=1)]
fn get_team_stats(hunt_key: String) -> Result<Page, Page> {
    let db = Database::new();
    let hunt = lookup_hunt(&hunt_key)?;
    let stats: Vec<_> = db.get_team_stats(hunt.id);
    Ok(xml("pages/puzzler/leaderboard.xml", vec!(&hunt, &stats)))
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
