use rocket::request::{FromForm, FormItems};

use data::{Team};
use database::Database;


// View Team //

#[derive(FromForm, Debug)]
pub struct ViewTeam {
    name: String,
    password: String
}

impl ViewTeam {
    pub fn view_team(&self, hunt_id: i32) -> Result<Team, String> {
        let db = Database::new();
        match db.get_team(hunt_id, &self.name, &self.password) {
            None       => Err("Team not found".to_string()),
            Some(team) => Ok(team)
        }
    }
}


// Register //
    
#[derive(Debug)]
pub struct Register {
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

impl Register {
    fn empty() -> Register {
        Register{
            name: "".to_string(),
            password: "".to_string(),
            password_verify: "".to_string(),
            members: vec!()
        }
    }
}

impl<'f> FromForm<'f> for Register {
    type Error = ();
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<Register, ()> {
        if !strict { return Err(()); }
        let mut reg = Register::empty();
        
        let mut member_name = "";
        for (key, value) in iter {
            match key.as_str() {
                "name"            => reg.name = value.to_string(),
                "password"        => reg.password = value.to_string(),
                "password_verify" => reg.password_verify = value.to_string(),
                "member_name"     => member_name = value,
                "member_email"    => {
                    let member = TeamMember{
                        name: member_name.to_string(),
                        email: value.to_string()
                    };
                    reg.members.push(member);
                },
                _ => return Err(())
            }
        }
        Ok(reg)
    }
}
