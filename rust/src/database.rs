use postgres::{Connection, TlsMode};
use postgres::types::ToSql;
use postgres::rows::{Rows};
use data::*;

pub struct Database {
    connection: Connection
}

impl Database {
    pub fn new() -> Database {
        match Connection::connect("postgresql://postgres:@localhost",
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

    fn query_one(&self, query: &str, params: &[&ToSql]) -> Rows {
        match self.connection.query(query, params) {
            Err(err) => panic!(
                format!("Failed to execute query: {}", err)),
            Ok(rows) => {
                if rows.len() != 1 {
                    panic!(
                        format!("Expected exactly one row, but found {}. Query: {}",
                                rows.len(), query));
                }
                rows
            }
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
        self.execute(Stats::drop_query(), &[]);
        // Initialize tables
        self.execute(Hunt::init_query(), &[]);
        self.execute(Wave::init_query(), &[]);
        self.execute(Puzzle::init_query(), &[]);
        self.execute(Hint::init_query(), &[]);
        self.execute(Team::init_query(), &[]);
        self.execute(Member::init_query(), &[]);
        self.execute(Guess::init_query(), &[]);
        self.execute(Solve::init_query(), &[]);
        self.execute(Stats::init_query(), &[]);
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
        self.execute(Stats::test_init_query(), &[]);
    }

    pub fn get_waves(&self, hunt: &str) -> Vec<Wave> {
        let hunt_id = self.get_hunt_id(&hunt);
        let mut waves = self.grab_waves(hunt_id);
        for wave in &mut waves {
            wave.puzzles = self.grab_puzzles(hunt_id, &wave.name);
        }
        waves
    }

    pub fn grab_waves(&self, hunt: i32) -> Vec<Wave> {
        let mut waves: Vec<Wave> = vec!();
        for row in &self.query(WAVE_QUERY, &[&hunt]) {
            waves.push(Wave::from_row(row))
        }
        waves
    }

    pub fn grab_hunt(&self, hunt: &str) -> Hunt {
        let rows = self.query_one(HUNT_QUERY, &[&hunt]);
        Hunt::from_row(rows.get(0))
    }

    pub fn grab_puzzles(&self, hunt: i32, wave: &str) -> Vec<Puzzle> {
        let mut puzzles: Vec<Puzzle> = vec!();
        for row in &self.query(PUZZLE_QUERY, &[&hunt, &wave]) {
            puzzles.push(Puzzle::from_row(row))
        }
        puzzles
    }

    pub fn get_hunt_id(&self, hunt: &str) -> i32 {
        let rows = self.query(HUNT_ID_QUERY, &[&hunt]);
        // TODO: error checking
        println!("Hunt {}", hunt);
        rows.get(0).get(0)
    }
}

//    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
//                 &[&me.name, &me.data]).unwrap();
//    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {

const PUZZLE_QUERY: &'static str =
    "select * from Puzzle where hunt = $1 and wave = $2;";

const WAVE_QUERY: &'static str =
    "select * from Wave where hunt = $1;";

const HUNT_ID_QUERY: &'static str =
    "select huntID from Hunt where key = $1";

const HUNT_QUERY: &'static str =
    "select * from Hunt where key = $1";
