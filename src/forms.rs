use std::collections::HashMap;
use rocket::request::{FromForm, FormItems};


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
        
        for (key, value) in iter {
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
        
        for (key, value) in iter {
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
    
#[derive(Debug)]
pub struct RegisterForm {
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

impl RegisterForm {
    fn empty() -> RegisterForm {
        RegisterForm{
            name: "".to_string(),
            password: "".to_string(),
            password_verify: "".to_string(),
            members: vec!()
        }
    }
}

impl<'f> FromForm<'f> for RegisterForm {
    type Error = String;
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<RegisterForm, String> {
        if !strict { return Err("Not strict".to_string()); }
        let mut form = RegisterForm::empty();
        
        let mut member_name = "";
        let mut first = true;
        for (key, value) in iter {
            match key.as_str() {
                "name"            => form.name = value.to_string(),
                "password"        => form.password = value.to_string(),
                "password_verify" => form.password_verify = value.to_string(),
                "member_name"     => member_name = value,
                "member_email"    => {
                    let member = TeamMember{
                        name: member_name.to_string(),
                        email: value.to_string()
                    };
                    // The first element is fake. Would be better to remove this on the frontend.
                    if first { first = false; } else { form.members.push(member); }
                },
                key => return Err(format!("Unrecognized key: {}", key))
            }
        }
        Ok(form)
    }
}


// Sign in //

#[derive(FromForm, Debug)]
pub struct SignInForm {
    pub name: String,
    pub password: String
}


// Update Team //

#[derive(Debug)]
pub struct UpdateTeamForm {
    pub name: String,
    pub members: Vec<TeamMember>
}

trait FromExpandableForm {
    type Child;
    fn empty() -> Self;
    fn parts(&mut self) -> HashMap<&str, &mut String>;
    fn empty_child() -> Self::Child;
    fn child_parts(&mut Self::Child) -> HashMap<&str, &mut String>;
}

impl FromExpandableForm for UpdateTeamForm {
    type Child = TeamMember;
    
    fn empty() -> UpdateTeamForm {
        UpdateTeamForm{
            name: "".to_string(),
            members: vec!()
        }
    }

    fn parts(&mut self) -> HashMap<&str, &mut String> {
        let mut map = HashMap::new();
        map.insert("name", &mut self.name);
        map
    }

    fn empty_child() -> TeamMember {
        TeamMember {
            name: "".to_string(),
            email: "".to_string()
        }
    }

    fn child_parts(member: &mut TeamMember) -> HashMap<&str, &mut String> {
        let mut map = HashMap::new();
        map.insert("name", &mut member.name);
        map.insert("email", &mut member.email);
        map
    }
}

impl<'f> FromForm<'f> for F where F: FromExpandableForm {
    type Error = String;
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<F, String> {
        if !strict { return Err("Not strict".to_string()); }
        let mut form = F::empty();
        let parts = form.parts();
        
        let mut first = true;
        let mut child = F::empty_child();
        let child_parts = F::child_parts(child);
        for (key, value) in iter {
            match parts.get(key.as_str()) {
                None => match child_parts.get(key
            }
            match key.as_str() {
                "name"            => form.name = value.to_string(),
                "member_name"     => member_name = value,
                "member_email"    => {
                    let member = TeamMember{
                        name: member_name.to_string(),
                        email: value.to_string()
                    };
                    // The first member is fake (b.c. expandable form)
                    if first { first = false; } else { form.members.push(member); }
                },
                key => return Err(format!("Unrecognized key: {}", key))
            }
        }
        Ok(form)
    }
}


impl UpdateTeamForm {
    fn empty() -> UpdateTeamForm {
        UpdateTeamForm{
            name: "".to_string(),
            members: vec!()
        }
    }
}

impl<'f> FromForm<'f> for UpdateTeamForm {
    type Error = String;
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<UpdateTeamForm, String> {
        if !strict { return Err("Not strict".to_string()); }
        let mut form = UpdateTeamForm::empty();
        
        let mut member_name = "";
        let mut first = true;
        for (key, value) in iter {
            match key.as_str() {
                "name"            => form.name = value.to_string(),
                "member_name"     => member_name = value,
                "member_email"    => {
                    let member = TeamMember{
                        name: member_name.to_string(),
                        email: value.to_string()
                    };
                    // The first member is fake (b.c. expandable form)
                    if first { first = false; } else { form.members.push(member); }
                },
                key => return Err(format!("Unrecognized key: {}", key))
            }
        }
        Ok(form)
    }
}
