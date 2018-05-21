use postgres::{Connection, TlsMode};
use postgres::types::ToSql;
use postgres::rows::{Rows};
use data::*;
use forms::*;

pub struct Database {
    connection: Connection
}

impl Database {
    pub fn new() -> Database {
        match Connection::connect("postgresql://postgres:pass@localhost",
                                  TlsMode::None) {
            Err(err) => panic!(
                format!("Could not connect to database: {}", err)),
            Ok(conn) => Database{ connection: conn }
        }
    }

    fn execute(&self, query: &str, params: &[&ToSql]) {
        match self.connection.execute(query, params) {
            Err(err) => panic!(
                format!("Failed to execute query: {}", err)),
            Ok(_) => ()
        }
    }

    fn query(&self, query: &str, params: &[&ToSql]) -> Rows {
        match self.connection.query(query, params) {
            Err(err) => panic!(
                format!("Failed to execute query: {}", err)),
            Ok(rows) => rows
        }
    }

    pub fn clear(&self) {
        // Drop existing tables
        self.execute(Hunt::drop_query(), &[]);
        self.execute(Wave::drop_query(), &[]);
        self.execute(Puzzle::drop_query(), &[]);
        self.execute(Hint::drop_query(), &[]);
        self.execute(Team::drop_query(), &[]);
        self.execute(Member::drop_query(), &[]);
        self.execute(Guess::drop_query(), &[]);
        self.execute(Solve::drop_query(), &[]);
        self.execute(Stat::drop_query(), &[]);
        // Initialize tables
        self.execute(Hunt::init_query(), &[]);
        self.execute(Wave::init_query(), &[]);
        self.execute(Puzzle::init_query(), &[]);
        self.execute(Hint::init_query(), &[]);
        self.execute(Team::init_query(), &[]);
        self.execute(Member::init_query(), &[]);
        self.execute(Guess::init_query(), &[]);
        self.execute(Solve::init_query(), &[]);
        self.execute(Stat::init_query(), &[]);
    }

    pub fn init_test(&self) {
        self.execute(Hunt::test_init_query(), &[]);
        self.execute(Wave::test_init_query(), &[]);
        self.execute(Puzzle::test_init_query(), &[]);
        self.execute(Hint::test_init_query(), &[]);
        self.execute(Team::test_init_query(), &[]);
        self.execute(Member::test_init_query(), &[]);
        self.execute(Guess::test_init_query(), &[]);
        self.execute(Solve::test_init_query(), &[]);
        self.execute(Stat::test_init_query(), &[]);
    }


    //// Hunts ////

    pub fn get_admin(&self, hunt_key: &str, password: &str, secret: &str) -> Option<Hunt> {
        if secret != "secret" {
            return None
        }
        let rows = self.query(
            "select * from Hunt where key = $1 and password = $3",
            &[&hunt_key, &password]);
        if rows.len() == 1 {
            Some(Hunt::from_row(rows.get(0)))
        } else {
            None
        }
    }

    
    //// Puzzles ////

    pub fn get_waves(&self, hunt_id: i32) -> Vec<Wave> {
        let mut waves: Vec<Wave> = vec!();
        let rows = self.query(
            "select * from Wave where hunt = $1;",
            &[&hunt_id]);
        for row in &rows {
            let mut wave = Wave::from_row(row);
            wave.puzzles = self.get_puzzles(hunt_id, &wave.name);
            waves.push(wave);
        }
        waves
    }

    pub fn get_hunts(&self) -> Vec<Hunt> {
        let mut hunts: Vec<Hunt> = vec!();
        for row in &self.query("select * from Hunt", &[]) {
            hunts.push(Hunt::from_row(row));
        }
        hunts
    }

    pub fn get_hunt(&self, hunt_key: &str) -> Hunt {
        let rows = self.query(
            "select * from Hunt where key = $1",
            &[&hunt_key]);
        if rows.len() != 1 {
            panic!("Did not find hunt"); // TODO: error handling
        }
        Hunt::from_row(rows.get(0))
    }

    pub fn get_hunt_by_id(&self, hunt_id: i32) -> Option<Hunt> {
        let rows = self.query(
            "select * from Hunt where huntID = $1",
            &[&hunt_id]);
        if rows.len() == 1 {
            Some(Hunt::from_row(rows.get(0)))
        } else {
            None
        }
    }

    pub fn get_puzzles(&self, hunt_id: i32, wave: &str) -> Vec<Puzzle> {
        let mut puzzles: Vec<Puzzle> = vec!();
        let rows = self.query(
            "select * from Puzzle where hunt = $1 and wave = $2;",
            &[&hunt_id, &wave]);
        for row in &rows {
            let mut puzzle = Puzzle::from_row(row);
            if puzzle.released {
                puzzle.hints = self.get_hints(hunt_id, &puzzle.name);
                puzzles.push(puzzle);
            }
        }
        puzzles
    }

    pub fn get_hints(&self, hunt_id: i32, puzzle: &str) -> Vec<Hint> {
        let mut hints: Vec<Hint> = vec!();
        let rows = self.query(
            "select * from Hint where hunt = $1 and puzzle = $2;",
            &[&hunt_id, &puzzle]);
        for row in &rows {
            hints.push(Hint::from_row(row));
        }
        hints
    }

    
    //// Teams ////

    pub fn team_exists(&self, hunt_id: i32, name: &str) -> bool {
        let rows = self.query(
            "select from Team where hunt = $1 and name = $2;",
            &[&hunt_id, &name]);
        rows.len() >= 1
    }

    fn fill_team_members(&self, team: &mut Team) {
        let rows = self.query(
            "select * from Member where TeamID = $1",
            &[&team.team_id]);
        let members = rows.iter().map(|row| Member::from_row(row)).collect();
        team.members = members;
    }

    pub fn get_team(&self, hunt_id: i32, name: &str, password: &str) -> Option<Team> {
        let rows = self.query(
            "select * from Team where hunt = $1 and name = $2 and password = $3",
            &[&hunt_id, &name, &password]);
        if rows.len() == 1 {
            let mut team = Team::from_row(rows.get(0));
            self.fill_team_members(&mut team);
            Some(team)
        } else {
            None
        }
    }

    pub fn get_team_by_id(&self, team_id: i32) -> Option<Team> {
        let rows = self.query(
            "select * from Team where teamID = $1",
            &[&team_id]);
        if rows.len() == 1 {
            let mut team = Team::from_row(rows.get(0));
            self.fill_team_members(&mut team);
            Some(team)
        } else {
            None
        }
    }

    pub fn register(&self, hunt_id: i32, form: &RegisterForm) -> Result<Team, String> {
        // Validate
        if form.password != form.password_verify {
            return Err("Passwords do not match".to_string())
        }
        if self.team_exists(hunt_id, &form.name) {
            return Err("A team of that name already exists.".to_string())
        }
        
        // Update
        let hunt = match self.get_hunt_by_id(hunt_id) {
            None => return Err("Failed to find hunt.".to_string()),
            Some(hunt) => hunt
        };
        let team_id: i32 = self.query(
            "insert into Team values (default, $1, $2, $3, $4) returning teamID",
            &[&hunt_id, &form.password, &form.name, &hunt.init_guesses]).get(0).get(0);
        for member in &form.members {
            println!("member: {:?}", member);
            self.execute(
                "insert into Member values ($1, $2, $3, $4)",
                &[&team_id, &hunt_id, &member.name, &member.email]);
        }

        // Return newly registred team
        match self.get_team(hunt_id, &form.name, &form.password) {
            None => Err("Failed to find team.".to_string()),
            Some(team) => {
                println!("team: {:?}", team);
                Ok(team)
            }
        }
    }

    pub fn update_team(&self, hunt_id: i32, form: &UpdateTeamForm) -> Result<Team, String> {
        // Validate
        let team = match self.get_team(hunt_id, &form.name, &form.password) {
            None => return Err("Team does not exist, or password does not match.".to_string()),
            Some(team) => team
        };
        
        // Update
        self.execute(
            "delete from Member where teamID = $1 and hunt = $2",
            &[&team.team_id, &hunt_id]);
        for member in &form.members {
            println!("member: {:?}", member);
            self.execute(
                "insert into Member values ($1, $2, $3, $4)",
                &[&team.team_id, &hunt_id, &member.name, &member.email]);
        }

        // Return updated team
        match self.get_team(hunt_id, &form.name, &form.password) {
            None => Err("Failed to find team.".to_string()),
            Some(team) => {
                println!("team: {:?}", team);
                Ok(team)
            }
        }
    }
}
    