use chrono::Utc;
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
        println!("Execute: {}\nwith: {:?}", query, params);
        match self.connection.execute(query, params) {
            Err(err) => panic!(
                format!("Failed to execute query\n`{}`\n{}", query, err)),
            Ok(_) => ()
        }
    }

    fn query(&self, query: &str, params: &[&ToSql]) -> Rows {
        println!("Query: {}\nwith: {:?}", query, params);
        match self.connection.query(query, params) {
            Err(err) => panic!(
                format!("Failed to execute query: {}", err)),
            Ok(rows) => {
                println!("Query ok.");
                rows
            }
        }
    }

    pub fn clear(&self) {
        // Drop existing tables
        self.execute(Site::drop_query(), &[]);
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
        self.execute(Site::init_query(), &[]);
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
        self.execute(Site::test_init_query(), &[]);
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

    
    //// Site ////

    pub fn get_site(&self) -> Site {
        let rows = self.query("select * from Site", &[]);
        if rows.len() != 1 {
            panic!("Invalid site information in database: there must be exactly one row in 'Site' table.");
        }
        Site::from_row(rows.get(0))
    }
    
    pub fn create_hunt(&self, form: &CreateHunt) -> Result<Hunt, String> {
        // Validate
        if form.password != form.password_verify {
            return Err("Passwords do not match".to_string())
        }
        if self.hunt_exists(&form.key) {
            return Err("A hunt of that name already exists.".to_string())
        }

        // Create hunt
        self.query(
            "insert into Hunt values(default, $1, $2, 4, 100, $3, false, false)",
            &[&form.name, &form.key, &form.password]);

        // Return newly created hunt
        Ok(self.get_hunt(&form.key))
    }


    //// Admin ////

    pub fn edit_hunt(&self, hunt_key: &str, form: &EditHunt) -> Result<Hunt, String> {
        // Update
        self.execute(
            "update Hunt set name = $2, teamSize = $3, initGuesses = $4, closed = $5, visible = $6 where key = $1",
            &[&hunt_key, &form.name, &form.team_size, &form.init_guesses, &form.closed, &form.visible]);

        // Return updated hunt
        Ok(self.get_hunt(&hunt_key))
    }
    
    pub fn get_all_teams(&self, hunt_id: i32) -> Vec<Team> {
        let rows = self.query(
            "select * from Team where Hunt = $1",
            &[&hunt_id]);
        rows.into_iter().map(|row| {
            let mut team = Team::from_row(row);
            self.fill_team_members(&mut team);
            team
        }).collect()
    }


    
    //// Hunts ////

    pub fn get_admin(&self, hunt_key: &str, password: &str) -> Option<Hunt> {
        let rows = self.query(
            "select * from Hunt where key = $1 and password = $2",
            &[&hunt_key, &password]);
        if rows.len() == 1 {
            Some(Hunt::from_row(rows.get(0)))
        } else {
            None
        }
    }

    
    //// Puzzles ////

    pub fn get_wave_infos(&self, hunt_id: i32) -> Vec<WaveInfo> {
        self.get_waves(hunt_id).into_iter().map(|wave| {
            WaveInfo {
                puzzles: self.get_puzzle_infos(hunt_id, &wave.name),
                name: wave.name,
                time: wave.time,
                guesses: wave.guesses,
                released: false // TODO: calculate
            }
        }).collect()
    }

    pub fn get_waves(&self, hunt_id: i32) -> Vec<Wave> {
        let rows = self.query(
            "select * from Wave where hunt = $1;",
            &[&hunt_id]);
        rows.iter().map(Wave::from_row).collect()
    }

    fn get_wave(&self, hunt_id: i32, wave_name: &str) -> Wave {
        let rows = self.query(
            "select * from Wave where hunt = $1 and name = $2;",
            &[&hunt_id, &wave_name]);
        if rows.len() != 1 {
            panic!("Wave {} not found (or not unique)", wave_name);
        }
        Wave::from_row(rows.get(0))
    }

    pub fn set_waves(&self, hunt_id: i32, waves: &Vec<Wave>) {
        self.execute("delete from Wave where hunt = $1", &[&hunt_id]);
        for wave in waves {
            let utc = wave.time.with_timezone(&Utc);
            self.execute("insert into Wave values ($1, $2, $3, $4)",
                         &[&wave.name, &hunt_id, &utc, &wave.guesses]);
        }
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
            panic!("Did not find hunt {}", hunt_key); // TODO: error handling
        }
        Hunt::from_row(rows.get(0))
    }

    pub fn hunt_exists(&self, hunt_key: &str) -> bool {
        let rows = self.query(
            "select * from Hunt where key = $1;",
            &[&hunt_key]);
        rows.len() >= 1
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

    pub fn get_puzzle_infos(&self, hunt_id: i32, wave: &str) -> Vec<PuzzleInfo> {
        let rows = self.query(
            "select * from Puzzle where hunt = $1 and wave = $2;",
            &[&hunt_id, &wave]);
        let mut puzzles = vec!();
        for row in &rows {
            let puzzle = Puzzle::from_row(row);
            let wave = self.get_wave(hunt_id, &puzzle.wave);
            if wave.is_released() {
                puzzles.push(PuzzleInfo {
                    hints: self.get_hints(hunt_id, &puzzle.name),
                    name: puzzle.name,
                    number: puzzle.number,
                    hunt: hunt_id,
                    base_points: puzzle.base_points,
                    current_points: puzzle.base_points, // TODO: calculate
                    answer: puzzle.answer,
                    wave: puzzle.wave,
                    key: puzzle.key,
                });
            }
        }
        puzzles
    }

    pub fn get_all_puzzles(&self, hunt_id: i32) -> Vec<Puzzle> {
        let mut puzzles: Vec<Puzzle> = vec!();
        let rows = self.query(
            "select * from Puzzle where hunt = $1;",
            &[&hunt_id]);
        for row in &rows {
            puzzles.push(Puzzle::from_row(row))
        }
        puzzles
    }

    pub fn set_puzzles(&self, hunt_id: i32, puzzles: &Vec<Puzzle>) {
        self.execute("delete from Puzzle where hunt = $1", &[&hunt_id]);
        for puzzle in puzzles {
            self.execute("insert into Puzzle values ($1, $2, $3, $4, $5, $6, $7)",
                         &[&puzzle.name, &puzzle.number, &hunt_id,
                           &puzzle.base_points, &puzzle.answer,
                           &puzzle.wave, &puzzle.key]);
        }
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

    pub fn set_hints(&self, hunt_id: i32, hints: &Vec<Hint>) {
        self.execute("delete from Hint where hunt = $1", &[&hunt_id]);
        for hint in hints {
            self.execute("insert into Hint values ($1, $2, $3, $4, $5, $6, $7)",
                         &[&hint.hint, &hint.puzzle, &hint.number,
                           &hunt_id, &hint.penalty, &hint.wave, &hint.key]);
        }
    }

    pub fn get_all_hints(&self, hunt_id: i32) -> Vec<Hint> {
        let mut hints: Vec<Hint> = vec!();
        let rows = self.query(
            "select * from Hint where hunt = $1;",
            &[&hunt_id]);
        for row in &rows {
            hints.push(Hint::from_row(row));
        }
        hints
    }

    pub fn get_hint(&self, hunt_id: i32, hint_key: &str) -> Option<Hint> {
        let rows = self.query(
            "select * from Hint where hunt = $1 and key = $2;",
            &[&hunt_id, &hint_key]);
        if rows.len() == 1 {
            Some(Hint::from_row(rows.get(0)))
        } else {
            None
        }
    }

    
    //// Teams ////

    pub fn team_exists(&self, hunt_id: i32, name: &str) -> bool {
        let rows = self.query(
            "select * from Team where hunt = $1 and name = $2;",
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

    pub fn authenticate_team(&self, hunt_id: i32, name: &str, password: &str) -> Option<i32> {
        let rows = self.query(
            "select teamID from Team where hunt = $1 and name = $2 and password = $3",
            &[&hunt_id, &name, &password]);
        if rows.len() == 1 {
            Some(rows.get(0).get(0))
        } else {
            None
        }
    }
    
    pub fn get_team(&self, hunt_id: i32, name: &str) -> Option<Team> {
        let rows = self.query(
            "select * from Team where hunt = $1 and name = $2",
            &[&hunt_id, &name]);
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

    pub fn register(&self, hunt_id: i32, form: &Register) -> Result<Team, String> {
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
        match self.get_team(hunt_id, &form.name) {
            None => Err("Failed to find team.".to_string()),
            Some(team) => {
                println!("team: {:?}", team);
                Ok(team)
            }
        }
    }

    pub fn update_team(&self, hunt_id: i32, form: &UpdateTeam) -> Result<Team, String> {
        // Validate
        let team = match self.get_team(hunt_id, &form.name) {
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
        match self.get_team(hunt_id, &form.name) {
            None => Err("Failed to find team.".to_string()),
            Some(team) => {
                println!("team: {:?}", team);
                Ok(team)
            }
        }
    }
}
