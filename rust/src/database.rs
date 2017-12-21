use postgres::{Connection, TlsMode};
use postgres::types::ToSql;
use postgres::rows::{Rows};
use data::*;

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

    
    //// Puzzles ////

    pub fn get_waves(&self, hunt_id: i32) -> Vec<Wave> {
        let mut waves: Vec<Wave> = vec!();
        for row in &self.query(WAVE_QUERY, &[&hunt_id]) {
            let mut wave = Wave::from_row(row);
            wave.puzzles = self.get_puzzles(hunt_id, &wave.name);
            waves.push(wave);
        }
        waves
    }

    pub fn get_hunt(&self, hunt_key: &str) -> Hunt {
        let rows = self.query(HUNT_QUERY, &[&hunt_key]);
        if rows.len() != 1 {
            panic!("Did not find hunt"); // TODO: error handling
        }
        Hunt::from_row(rows.get(0))
    }

    pub fn get_puzzles(&self, hunt_id: i32, wave: &str) -> Vec<Puzzle> {
        let mut puzzles: Vec<Puzzle> = vec!();
        for row in &self.query(PUZZLE_QUERY, &[&hunt_id, &wave]) {
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
        for row in &self.query(HINT_QUERY, &[&hunt_id, &puzzle]) {
            hints.push(Hint::from_row(row));
        }
        hints
    }

    
    //// Teams ////

    pub fn team_exists(&self, hunt_id: i32, name: &str) -> bool {
        let rows = self.query(TEAM_EXISTS_QUERY, &[&hunt_id, &name]);
        rows.len() >= 1
    }

    fn grab_team(&self, hunt_id: i32, name: &str, password: &str) -> Option<Team> {
        let rows = self.query(TEAM_QUERY, &[&hunt_id, &name, &password]);
        if rows.len() == 1 {
            Some(Team::from_row(rows.get(0)))
        } else {
            None
        }
    }

    pub fn get_team(&self, hunt_id: i32, name: &str, password: &str) -> Option<Team> {
        let mut team = match self.grab_team(hunt_id, name, password) {
            None => return None,
            Some(team) => team
        };
        let rows = self.query(MEMBER_QUERY, &[&hunt_id, &team.team_id]);
        let members = rows.iter().map(|row| Member::from_row(row)).collect();
        team.members = members;
        Some(team)
    }
}


const HUNT_QUERY: &'static str =
    "select * from Hunt where key = $1";

const WAVE_QUERY: &'static str =
    "select * from Wave where hunt = $1;";

const PUZZLE_QUERY: &'static str =
    "select * from Puzzle where hunt = $1 and wave = $2;";

const HINT_QUERY: &'static str =
    "select * from Hint where hunt = $1 and puzzle = $2;";

const TEAM_EXISTS_QUERY: &'static str =
    "select () from Team where hunt = $1 and name = $2;";

const TEAM_QUERY: &'static str =
    "select * from Team where hunt = $1 and name = $2 and password = $3";

const MEMBER_QUERY: &'static str =
    "select * from Member where hunt = $1 and TeamID = $2";
