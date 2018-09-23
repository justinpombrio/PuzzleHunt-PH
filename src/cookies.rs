use data::*;
use database::Database;

use rocket::Outcome;
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, Request, FromRequest};

pub struct Puzzler {
    pub team_id: i32
}

impl<'a, 'r> FromRequest<'a, 'r> for Puzzler {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Puzzler, ()> {
        let mut cookies = request.cookies();
        if let Some(ref cookie) = cookies.get_private("team_id") {
            if let Ok(team_id) = cookie.value().parse() {
                return Outcome::Success(Puzzler{ team_id: team_id });
            }
        }
        Outcome::Forward(())
    }
}

impl Database {

    pub fn signin_team(&self, cookies: &mut Cookies,
                       hunt_id: i32, team_name: &str, password: &str) -> bool {
        let team_id = match self.authenticate_team(hunt_id, team_name, password) {
            None => return false,
            Some(id) => id
        };
        cookies.remove_private(Cookie::named("team_id"));
        cookies.add_private(Cookie::new("team_id", format!("{}", team_id)));
        println!("Added cookie {}={}", "team_id", format!("{}", team_id));
        true
    }

    pub fn signin_admin(&self, cookies: &mut Cookies,
                        hunt_key: &str, password: &str) -> bool {
        let hunt = match self.get_admin(hunt_key, password) {
            None => return false,
            Some(hunt) => hunt
        };
        cookies.remove_private(Cookie::named("hunt_id"));
        cookies.add_private(Cookie::new("hunt_id", format!("{}", hunt.id)));
        true
    }

    pub fn signout_team(&self, cookies: &mut Cookies) {
        cookies.remove_private(Cookie::named("team_id"));
    }

    pub fn signout_admin(&self, cookies: &mut Cookies) {
        cookies.remove_private(Cookie::named("hunt_id"));
    }

    pub fn signedin_team(&self, cookies: &mut Cookies) -> Option<Team> {
        if let Some(ref cookie) = cookies.get_private("team_id") {
            if let Ok(team_id) = cookie.value().parse() {
                return self.get_team_by_id(team_id)
            }
        }
        None
    }

    pub fn signedin_admin(&self, cookies: &mut Cookies) -> Option<Hunt> {
        if let Some(ref cookie) = cookies.get_private("hunt_id") {
            if let Ok(hunt_id) = cookie.value().parse() {
                return self.get_hunt_by_id(hunt_id)
            }
        }
        None
    }
}
