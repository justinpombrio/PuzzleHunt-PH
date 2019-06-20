use std::collections::HashMap;
use rocket::request::Form;
use crate::expandable_form::{RegularForm, ExpandableForm, RegularFormToForm, ExpandableFormToForm};
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
pub type CreateHuntForm = Form<RegularFormToForm<CreateHunt>>;

impl RegularForm for CreateHunt {
    fn parts() -> Vec<&'static str> {
        vec!("key", "name", "password", "passwordVerify", "secret")
    }

    fn new(map: &HashMap<String, String>) -> CreateHunt {
        CreateHunt {
            key:             map["key"].to_string(),
            name:            map["name"].to_string(),
            password:        map["password"].to_string(),
            password_verify: map["password_verify"].to_string(),
            secret:          map["secret"].to_string()
        }
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
pub type EditHuntForm = Form<RegularFormToForm<EditHunt>>;

impl RegularForm for EditHunt {
    fn parts() -> Vec<&'static str> {
        vec!("key", "name", "team_size", "init_guesses", "closed", "visible")
    }

    fn new(map: &HashMap<String, String>) -> EditHunt {
        EditHunt {
            name:         map["name"].to_string(),
            team_size:    map["team_size"].parse().expect("Failed to parse 'teamSize'"),
            init_guesses: map["init_guesses"].parse().expect("Failed to parse 'initGuesses'"),
            closed:       read_form_boolean(map, "closed"),
            visible:      read_form_boolean(map, "visible")
        }
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
pub type WavesForm = Form<ExpandableFormToForm<Waves>>;

impl ExpandableForm for Waves {
    type Member = Wave;

    fn parts() -> Vec<&'static str> {
        vec!()
    }

    fn member_parts() -> Vec<&'static str> {
        vec!("name", "time", "guesses")
    }

    fn new_member(map: &HashMap<String, String>) -> Wave {
        Wave {
            name: map["name"].to_string(),
            time: map["time"].parse().expect("Could not parse 'datetime'"),
            guesses: map["guesses"].parse().expect("Could not parse 'guesses'")
        }
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
pub type PuzzlesForm = Form<ExpandableFormToForm<Puzzles>>;

impl ExpandableForm for Puzzles {
    type Member = Puzzle;

    fn parts() -> Vec<&'static str> {
        vec!()
    }
    
    fn member_parts() -> Vec<&'static str> {
        vec!("name", "answer", "wave", "key")
    }
    
    fn new_member(map: &HashMap<String, String>) -> Puzzle {
        Puzzle {
            name: map["name"].to_string(),
            hunt: 0,
            answer: map["answer"].to_string(),
            wave: map["wave"].to_string(),
            key: map["key"].to_string()
        }
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
pub type HintsForm = Form<ExpandableFormToForm<Hints>>;

impl ExpandableForm for Hints {
    type Member = Hint;

    fn parts() -> Vec<&'static str> {
        vec!()
    }

    fn member_parts() -> Vec<&'static str> {
        vec!("hint", "puzzle_key", "number", "hunt", "wave", "key")
    }
    
    fn new_member(map: &HashMap<String, String>) -> Hint {
        Hint {
            hint: map["hint"].to_string(),
            puzzle_key: map["puzzle_key"].to_string(),
            number: map["number"].parse()
                .expect("Could not parse `number`"),
            hunt: 0,
            wave: map["wave"].to_string(),
            key: map["key"].to_string()
        }
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
pub type CreateTeamForm = Form<ExpandableFormToForm<CreateTeam>>;

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

    fn new_member(map: &HashMap<String, String>) -> TeamMember {
        TeamMember {
            name: map["member_name"].to_string(),
            email: map["member_email"].to_string()
        }
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
pub type UpdateTeamForm = Form<ExpandableFormToForm<UpdateTeam>>;

impl ExpandableForm for UpdateTeam {
    type Member = TeamMember;

    fn parts() -> Vec<&'static str> {
        vec!("name", "guesses")
    }

    fn member_parts() -> Vec<&'static str> {
        vec!("member_name", "member_email")
    }

    fn new_member(map: &HashMap<String, String>) -> TeamMember {
        TeamMember {
            name: map["member_name"].to_string(),
            email: map["member_email"].to_string()
        }
    }

    fn new(map: &HashMap<String, String>, members: Vec<TeamMember>) -> UpdateTeam {
        UpdateTeam {
            name: map["name"].to_string(),
            members: members
        }
    }
}

fn read_form_boolean(map: &HashMap<String, String>, key: &str) -> bool {
    match map.get(key) {
        Some(val) => val == "on",
        None => false
    }
}
