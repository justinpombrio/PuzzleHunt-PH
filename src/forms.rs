use rocket::request::{FromForm, FormItems};

use data::{Team};
use database::Database;


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
    type Error = ();
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<RegisterForm, ()> {
        if !strict { return Err(()); }
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
                _ => return Err(())
            }
        }
        Ok(form)
    }
}


// Sign In //

#[derive(FromForm, Debug)]
pub struct SignInForm {
    pub name: String,
    pub password: String
}


// Update Team //

#[derive(Debug)]
pub struct UpdateTeamForm {
    pub name: String,
    pub password: String,
    pub members: Vec<TeamMember>
}

impl UpdateTeamForm {
    fn empty() -> UpdateTeamForm {
        UpdateTeamForm{
            name: "".to_string(),
            password: "".to_string(),
            members: vec!()
        }
    }
}

impl<'f> FromForm<'f> for UpdateTeamForm {
    type Error = ();
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<UpdateTeamForm, ()> {
        if !strict { return Err(()); }
        let mut form = UpdateTeamForm::empty();
        
        let mut member_name = "";
        let mut first = true;
        for (key, value) in iter {
            match key.as_str() {
                "name"            => form.name = value.to_string(),
                "password"        => form.password = value.to_string(),
                "member_name"     => member_name = value,
                "member_email"    => {
                    let member = TeamMember{
                        name: member_name.to_string(),
                        email: value.to_string()
                    };
                    // The first member is fake (b.c. expandable form)
                    if first { first = false; } else { form.members.push(member); }
                },
                _ => return Err(())
            }
        }
        Ok(form)
    }
}

impl UpdateTeamForm {
    pub fn update_team(&self, hunt_id: i32) -> Result<Team, String> {
        let db = Database::new();
        match db.get_team(hunt_id, &self.name, &self.password) {
            None       => Err("Team not found".to_string()),
            Some(team) => Ok(team)
        }
    }
}
