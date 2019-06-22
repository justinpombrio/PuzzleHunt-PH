use std::collections::HashMap;
use rocket::request::Form;
use chrono::{Local, DateTime};
use crate::expandable_form::{RegularForm, ExpandableForm, RegularFormResult, ExpandableFormResult};
use crate::data::{Wave, Puzzle, Hint};


// Create Hunt //

#[derive(Debug)]
pub struct CreateHunt {
    pub key: String,
    pub name: String,
    pub password: String,
    pub password_verify: String,
    pub secret: String
}
pub type CreateHuntForm = Form<RegularFormResult<CreateHunt>>;

impl RegularForm for CreateHunt {
    fn parts() -> Vec<&'static str> {
        vec!("key", "name", "password", "passwordVerify", "secret")
    }

    fn new(map: &HashMap<String, String>) -> Result<CreateHunt, String> {
        Ok(CreateHunt {
            key:             read_string(map, "key")?,
            name:            read_string(map, "name")?,
            password:        read_string(map, "password")?,
            password_verify: read_string(map, "password_verify")?,
            secret:          read_string(map, "secret")?
        })
    }
}


// Edit Hunt //

#[derive(Debug)]
pub struct EditHunt {
    pub name: String,
    pub team_size: i32,
    pub init_guesses: i32,
    pub closed: bool,
    pub visible: bool
}
pub type EditHuntForm = Form<RegularFormResult<EditHunt>>;

impl RegularForm for EditHunt {
    fn parts() -> Vec<&'static str> {
        vec!("key", "name", "team_size", "init_guesses", "closed", "visible")
    }

    fn new(map: &HashMap<String, String>) -> Result<EditHunt, String> {
        match Self::foo(map) {
            Ok(x) => Ok(x),
            Err(err) => {
                println!("ERROR");
                Err(err)
            }
        }
    }
}

impl EditHunt {
    fn foo(map: &HashMap<String, String>) -> Result<EditHunt, String> {
        println!("PARSING");
        Ok(EditHunt {
            name:         read_string(map, "name")?,
            team_size:    read_i32(map, "team_size")?,
            init_guesses: read_i32(map, "init_guesses")?,
            closed:       read_bool(map, "closed"),
            visible:      read_bool(map, "visible")
        })
    }
}


// Admin Signin //

#[derive(FromForm, Debug)]
pub struct AdminSignIn {
    pub hunt_key: String,
    pub password: String
}
pub type AdminSignInForm = Form<AdminSignIn>;

// Admin Edit Waves //

pub struct Waves {
    pub waves: Vec<Wave>
}
pub type WavesForm = Form<ExpandableFormResult<Waves>>;

impl ExpandableForm for Waves {
    type Member = Wave;

    fn parts() -> Vec<&'static str> {
        vec!()
    }

    fn member_parts() -> Vec<&'static str> {
        vec!("name", "time", "guesses")
    }

    fn new_member(map: &HashMap<String, String>) -> Result<Wave, String> {
        Ok(Wave {
            name: read_string(map, "name")?,
            time: read_datetime(map, "time")?,
            guesses: read_i32(map, "guesses")?
        })
    }

    fn new(_: &HashMap<String, String>, waves: Vec<Wave>) -> Waves {
        Waves {
            waves: waves
        }
    }
}

// Admin Edit Puzzles //

pub struct Puzzles {
    pub puzzles: Vec<Puzzle>
}
pub type PuzzlesForm = Form<ExpandableFormResult<Puzzles>>;

impl ExpandableForm for Puzzles {
    type Member = Puzzle;

    fn parts() -> Vec<&'static str> {
        vec!()
    }
    
    fn member_parts() -> Vec<&'static str> {
        vec!("name", "answer", "wave", "key")
    }
    
    fn new_member(map: &HashMap<String, String>) -> Result<Puzzle, String> {
        Ok(Puzzle {
            name: read_string(map, "name")?,
            hunt: 0,
            answer: read_string(map, "answer")?,
            wave: read_string(map, "wave")?,
            key: read_string(map, "key")?
        })
    }
    
    fn new(_: &HashMap<String, String>, puzzles: Vec<Puzzle>) -> Puzzles {
        Puzzles {
            puzzles: puzzles
        }
    }
}

// Admin Edit Hints //

pub struct Hints {
    pub hints: Vec<Hint>
}
pub type HintsForm = Form<ExpandableFormResult<Hints>>;

impl ExpandableForm for Hints {
    type Member = Hint;

    fn parts() -> Vec<&'static str> {
        vec!()
    }

    fn member_parts() -> Vec<&'static str> {
        vec!("hint", "puzzle_name", "number", "hunt", "wave", "key")
    }
    
    fn new_member(map: &HashMap<String, String>) -> Result<Hint, String> {
        Ok(Hint {
            hint: read_string(map, "hint")?,
            puzzle_name: read_string(map, "puzzle_name")?,
            number: read_i32(map, "number")?,
            hunt: 0,
            wave: read_string(map, "wave")?,
            key: read_string(map, "key")?
        })
    }
    
    fn new(_: &HashMap<String, String>, hints: Vec<Hint>) -> Hints {
        Hints {
            hints: hints
        }
    }
}



// Create Team //

#[derive(Debug)]
pub struct CreateTeam {
    pub name: String,
    pub password: String,
    pub password_verify: String,
    pub members: Vec<TeamMember>
}
pub type CreateTeamForm = Form<ExpandableFormResult<CreateTeam>>;

#[derive(Debug)]
pub struct TeamMember {
    pub name: String,
    pub email: String
}

impl ExpandableForm for CreateTeam {
    type Member = TeamMember;

    fn parts() -> Vec<&'static str> {
        vec!("name", "password", "password_verify")
    }

    fn member_parts() -> Vec<&'static str> {
        vec!("member_name", "member_email")
    }

    fn new_member(map: &HashMap<String, String>) -> Result<TeamMember, String> {
        Ok(TeamMember {
            name: read_string(map, "member_name")?,
            email: read_string(map, "member_email")?
        })
    }

    fn new(map: &HashMap<String, String>, members: Vec<TeamMember>) -> CreateTeam {
        CreateTeam {
            name: map["name"].to_string(),
            password: map["password"].to_string(),
            password_verify: map["password"].to_string(),
            members: members
        }
    }
}



// Sign in //

#[derive(FromForm, Debug)]
pub struct SignIn {
    pub name: String,
    pub password: String
}
pub type SignInForm = Form<SignIn>;


// Update Team //

#[derive(Debug)]
pub struct UpdateTeam {
    pub name: String,
    pub members: Vec<TeamMember>
}
pub type UpdateTeamForm = Form<ExpandableFormResult<UpdateTeam>>;

impl ExpandableForm for UpdateTeam {
    type Member = TeamMember;

    fn parts() -> Vec<&'static str> {
        vec!("name", "guesses")
    }

    fn member_parts() -> Vec<&'static str> {
        vec!("member_name", "member_email")
    }

    fn new_member(map: &HashMap<String, String>) -> Result<TeamMember, String> {
        Ok(TeamMember {
            name: read_string(map, "member_name")?,
            email: read_string(map, "member_email")?
        })
    }

    fn new(map: &HashMap<String, String>, members: Vec<TeamMember>) -> UpdateTeam {
        UpdateTeam {
            name: map["name"].to_string(),
            members: members
        }
    }
}


// Submit Answer //

#[derive(Debug)]
pub struct SubmitAnswer {
    pub guess: String
}
pub type SubmitAnswerForm = Form<RegularFormResult<SubmitAnswer>>;

impl RegularForm for SubmitAnswer {
    fn parts() -> Vec<&'static str> {
        vec!("guess")
    }

    fn new(map: &HashMap<String, String>) -> Result<SubmitAnswer, String> {
        Ok(SubmitAnswer {
            guess: read_string(map, "guess")?
        })
    }
}


// Utility //

fn read_bool(map: &HashMap<String, String>, key: &str) -> bool {
    match map.get(key) {
        Some(val) => val == "on",
        None => false
    }
}

fn read_string(map: &HashMap<String, String>, key: &str) -> Result<String, String> {
    match map.get(key) {
        Some(val) => Ok(val.to_string()),
        None => Err(format!("Failed to find form item '{}'.", key))
    }
}

fn read_i32(map: &HashMap<String, String>, key: &str) -> Result<i32, String> {
    match map.get(key) {
        Some(val) => match val.parse() {
            Ok(n) => Ok(n),
            Err(_) => Err(format!("Failed to parse value '{}' as a number.", val))
        },
        None => Err(format!("Key '{}' not found.", key))
    }
}

fn read_datetime(map: &HashMap<String, String>, key: &str) -> Result<DateTime<Local>, String> {
    match map.get(key) {
        Some(val) => match val.parse() {
            Ok(dt) => Ok(dt),
            Err(_) => Err(format!("Failed to parse value '{}' as a number.", val))
        },
        None => Err(format!("Key '{}' not found.", key))
    }
}
