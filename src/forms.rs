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
    type Error = ();
    fn from_form(iter: &mut FormItems<'f>, strict: bool) -> Result<CreateHuntForm, ()> {
        if !strict { return Err(()); }
        let mut form = CreateHuntForm::empty();
        
        for (key, value) in iter {
            match key.as_str() {
                "key"             => form.key = value.to_string(),
                "name"            => form.name = value.to_string(),
                "password"        => form.password = value.to_string(),
                "password_verify" => form.password_verify = value.to_string(),
                "secret"          => form.secret = value.to_string(),
                _ => return Err(())
            }
        }
        Ok(form)
    }
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

impl UpdateTeamForm {
    fn empty() -> UpdateTeamForm {
        UpdateTeamForm{
            name: "".to_string(),
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
