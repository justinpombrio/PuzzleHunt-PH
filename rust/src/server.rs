use std::path::{Path, PathBuf};
use std::fs::File;

use rocket;
use rocket::response::content::{Xml};
use rocket::request::{Form, FromForm, FormItems};

use util::*;
use data::{AddToData, build_data};
use database::Database;


fn serve_file<P : AsRef<Path>>(path: P) -> Option<File> {
    File::open(path).ok()
}

//fn render_xml<P : AsRef<Path>>(path: P, data: mustache::Data) -> Xml<String> {
//    Xml(render_mustache(path, data))
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
/*
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
*/
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

#[derive(FromForm, Debug)]
struct ViewTeamInput {
    name: String,
    password: String
}

#[post("/<hunt_key>/view-team.xml", rank=0, data="<input>")]
fn post_view_team(hunt_key: String, input: Form<ViewTeamInput>) -> Xml<String> {
    let input = input.into_inner();
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let team = match db.get_team(hunt.id, &input.name, &input.password) {
        None => panic!("Team not found"), // TODO: error handling
        Some(team) => team
    };
    render_xml("your-team.xml", vec!(&hunt, &team))
}

#[get("/<hunt_key>/register.xml", rank=0)]
fn get_register(hunt_key: String) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    render_xml("register.xml", vec!(&hunt))
}

#[derive(Debug)]
struct RegistrationInput {
    name: String,
    password: String,
    password_verify: String,
    members: Vec<TeamMemberInput>
}

#[derive(Debug)]
struct TeamMemberInput {
    name: String,
    email: String
}

impl<'f> FromForm<'f> for RegistrationInput {
    type Error = ();
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<RegistrationInput, ()> {
        if !strict { return Err(()); }
        let mut name = "";
        let mut password = "";
        let mut password_verify = "";
        let mut member_name = "";
        let mut members = vec!();
        for (key, value) in iter {
            match key.as_str() {
                "name" => name = value,
                "password" => password = value,
                "password_verify" => password_verify = value,
                "member_name" => member_name = value,
                "member_email" => {
                    let member = TeamMemberInput{
                        name: member_name.to_string(),
                        email: value.to_string()
                    };
                    members.push(member);
                },
                _ => return Err(())
            }
        }
        Ok(RegistrationInput{
            name: name.to_string(),
            password: password.to_string(),
            password_verify: password_verify.to_string(),
            members: members
        })
    }
}

#[post("/<hunt_key>/register.xml", data="<input>")]
fn post_register(hunt_key: String, input: Form<RegistrationInput>) -> Xml<String> {
    let db = Database::new();
    let hunt = db.get_hunt(&hunt_key);
    let input = input.into_inner();
    if input.password != input.password_verify {
        panic!("Passwords do not match");
    }
    println!("Team Registration: {:?}", input);
    render_xml("your-team.xml", vec!(&hunt))
}


pub fn start() {
    rocket::ignite().mount("/", routes![
        get_css, get_ph, get_js,
        get_hunt, get_puzzles, get_team, get_register, get_view_team,
        post_register, post_view_team
    ]).launch();
}
