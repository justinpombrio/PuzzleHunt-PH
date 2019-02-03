use std::collections::HashMap;
use rocket::request::{FromForm, FormItems};
use crate::expandable_form::*;


// Create Hunt //

#[derive(Debug)]
pub struct CreateHuntForm {
    pub key: String,
    pub name: String,
    pub password: String,
    pub password_verify: String,
    pub secret: String
}

impl CreateHuntForm {
    fn empty() -> CreateHuntForm {
        CreateHuntForm{
            key: "".to_string(),
            name: "".to_string(),
            password: "".to_string(),
            password_verify: "".to_string(),
            secret: "".to_string()
        }
    }
}

impl<'f> FromForm<'f> for CreateHuntForm {
    type Error = String;
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<CreateHuntForm, String> {
        if !strict { return Err("Not strict".to_string()); }
        let mut form = CreateHuntForm::empty();
        
        for (key, value) in iter.map(|f| (f.key, f.value)) {
            match key.as_str() {
                "key"             => form.key = value.to_string(),
                "name"            => form.name = value.to_string(),
                "password"        => form.password = value.to_string(),
                "password_verify" => form.password_verify = value.to_string(),
                "secret"          => form.secret = value.to_string(),
                key => return Err(format!("Unrecognized key: {}", key))
            }
        }
        Ok(form)
    }
}


// Edit Hunt //

#[derive(Debug)]
pub struct EditHuntForm {
    pub name: String,
    pub team_size: i32,
    pub init_guesses: i32,
    pub closed: bool,
    pub visible: bool
}

impl EditHuntForm {
    fn empty() -> EditHuntForm {
        EditHuntForm{
            name: "".to_string(),
            team_size: 0,
            init_guesses: 0,
            closed: true,
            visible: false
        }
    }
}

impl<'f> FromForm<'f> for EditHuntForm {
    type Error = String;
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<EditHuntForm, String> {
        if !strict { return Err("Not Strict".to_string()); }
        let mut form = EditHuntForm::empty();
        
        for (key, value) in iter.map(|f| (f.key, f.value)) {
            let value = value.url_decode()
                .expect(&format!("Failed to decode value: {:?}", value))
                .to_string();
            match key.as_str() {
                "name"         => form.name = value,
                "teamSize"     => form.team_size = value.parse()
                    .expect("Failed to parse 'teamSize'"),
                "initGuesses"  => form.init_guesses = value.parse()
                    .expect("Failed to parse 'initGuesses'"),
                "closed"       => form.closed = value.parse()
                    .expect("Failed to parse 'closed'"),
                "visible"      => form.visible = value.parse()
                    .expect("Failed to parse 'visible'"),
                key => return Err(format!("Unrecognized key: {}", key))
            }
        }
        Ok(form)
    }
}


// Admin Signin //

#[derive(FromForm, Debug)]
pub struct AdminSignInForm {
    pub hunt_key: String,
    pub password: String
}


// Register //

pub type RegisterForm = ExpandableFormToForm<RegisterFormRaw>;
    
#[derive(Debug)]
pub struct RegisterFormRaw {
    pub name: String,
    pub password: String,
    pub password_verify: String,
    pub members: Vec<TeamMember>
}

#[derive(Debug)]
pub struct TeamMember {
    pub name: String,
    pub email: String
}

impl FromExpandableForm for RegisterFormRaw {
    type Member = TeamMember;

    fn parts() -> Vec<&'static str> {
        vec!("name", "password", "password_verify")
    }

    fn member_parts() -> Vec<&'static str> {
        vec!("member_name", "member_email")
    }

    fn new_member(map: &HashMap<&str, &str>) -> TeamMember {
        TeamMember {
            name: map["member_name"].to_string(),
            email: map["member_email"].to_string()
        }
    }

    fn new(map: &HashMap<&str, &str>, members: Vec<TeamMember>) -> RegisterFormRaw {
        RegisterFormRaw {
            name: map["name"].to_string(),
            password: map["password"].to_string(),
            password_verify: map["password"].to_string(),
            members: members
        }
    }
}



// Sign in //

#[derive(FromForm, Debug)]
pub struct SignInForm {
    pub name: String,
    pub password: String
}


// Update Team //

pub type UpdateTeamForm = ExpandableFormToForm<UpdateTeamFormRaw>;

#[derive(Debug)]
pub struct UpdateTeamFormRaw {
    pub name: String,
    pub members: Vec<TeamMember>
}


impl FromExpandableForm for UpdateTeamFormRaw {
    type Member = TeamMember;

    fn parts() -> Vec<&'static str> {
        vec!("name")
    }

    fn member_parts() -> Vec<&'static str> {
        vec!("member_name", "member_email")
    }

    fn new_member(map: &HashMap<&str, &str>) -> TeamMember {
        TeamMember {
            name: map["member_name"].to_string(),
            email: map["member_email"].to_string()
        }
    }

    fn new(map: &HashMap<&str, &str>, members: Vec<TeamMember>) -> UpdateTeamFormRaw {
        UpdateTeamFormRaw {
            name: map["name"].to_string(),
            members: members
        }
    }
}
