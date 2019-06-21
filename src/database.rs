use chrono::{Utc, Local};
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
        // A single annoying index
        self.execute(Guess::index_query(), &[]);
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
    }

    
    //// Site ////

    pub fn get_site(&self) -> Site {
        let rows = self.query("select * from Site", &[]);
        if rows.len() != 1 {
            panic!("Invalid site information in database: there must be exactly one row in 'Site' table.");
        }
        Site::from_row(rows.get(0))
    }
    
    pub fn get_hunts(&self) -> Vec<Hunt> {
        let mut hunts: Vec<Hunt> = vec!();
        for row in &self.query("select * from Hunt", &[]) {
            hunts.push(Hunt::from_row(row));
        }
        hunts
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
        self.execute(
            "insert into Hunt values(default, $1, $2, 4, 100, $3, false, false)",
            &[&form.name, &form.key, &form.password]);

        // Return newly created hunt
        Ok(self.get_hunt(&form.key))
    }

    fn hunt_exists(&self, hunt_key: &str) -> bool {
        let rows = self.query(
            "select * from Hunt where key = $1;",
            &[&hunt_key]);
        rows.len() >= 1
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

    
    //// Authentication ////

    pub fn authenticate_admin(&self, hunt_key: &str, password: &str) -> Option<Hunt> {
        let rows = self.query(
            "select * from Hunt where key = $1 and password = $2",
            &[&hunt_key, &password]);
        if rows.len() == 1 {
            Some(Hunt::from_row(rows.get(0)))
        } else {
            None
        }
    }

    pub fn authenticate_team(&self, hunt_id: i32, name: &str, password: &str) -> Option<i32> {
        let rows = self.query(
            "select team_id from Team where hunt = $1 and name = $2 and password = $3",
            &[&hunt_id, &name, &password]);
        if rows.len() == 1 {
            Some(rows.get(0).get(0))
        } else {
            None
        }
    }

    pub fn get_hunt_by_id(&self, hunt_id: i32) -> Option<Hunt> {
        let rows = self.query(
            "select * from Hunt where id = $1",
            &[&hunt_id]);
        if rows.len() == 1 {
            Some(Hunt::from_row(rows.get(0)))
        } else {
            None
        }
    }

    pub fn get_team_by_id(&self, team_id: i32) -> Option<Team> {
        let rows = self.query(
            "select * from Team where team_id = $1",
            &[&team_id]);
        if rows.len() == 1 {
            let mut team = Team::from_row(rows.get(0));
            self.fill_team_members(&mut team);
            Some(team)
        } else {
            None
        }
    }



    //// Admin ////

    pub fn edit_hunt(&self, hunt_key: &str, form: &EditHunt) -> Result<Hunt, String> {
        // Update
        self.execute(
            "update Hunt set name = $2, team_size = $3, init_guesses = $4, closed = $5, visible = $6 where key = $1",
            &[&hunt_key, &form.name, &form.team_size, &form.init_guesses, &form.closed, &form.visible]);

        // Return updated hunt
        Ok(self.get_hunt(&hunt_key))
    }
    
    pub fn get_teams(&self, hunt_id: i32) -> Vec<Team> {
        let rows = self.query(
            "select * from Team where Hunt = $1",
            &[&hunt_id]);
        rows.into_iter().map(|row| {
            let mut team = Team::from_row(row);
            self.fill_team_members(&mut team);
            team
        }).collect()
    }

    pub fn get_waves(&self, hunt_id: i32) -> Vec<Wave> {
        let rows = self.query(
            "select * from Wave where hunt = $1;",
            &[&hunt_id]);
        rows.iter().map(Wave::from_row).collect()
    }

    fn get_wave(&self, hunt_id: i32, wave_name: &str) -> Option<Wave> {
        let rows = self.query(
            "select * from Wave where hunt = $1 and name = $2;",
            &[&hunt_id, &wave_name]);
        if rows.len() != 1 {
            None
        } else {
            Some(Wave::from_row(rows.get(0)))
        }
    }

    pub fn set_waves(&self, hunt_id: i32, waves: &Vec<Wave>) {
        self.execute("delete from Wave where hunt = $1", &[&hunt_id]);
        for wave in waves {
            let utc = wave.time.with_timezone(&Utc);
            self.execute("insert into Wave values ($1, $2, $3, $4)",
                         &[&wave.name, &hunt_id, &utc, &wave.guesses]);
        }
    }

    pub fn get_puzzles(&self, hunt_id: i32) -> Vec<Puzzle> {
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
            self.execute("insert into Puzzle values ($1, $2, $3, $4, $5)",
                         &[&puzzle.name, &hunt_id, &puzzle.answer.to_ascii_uppercase(),
                           &puzzle.wave, &puzzle.key]);
        }
    }

    pub fn get_hints(&self, hunt_id: i32) -> Vec<Hint> {
        let mut hints: Vec<Hint> = vec!();
        let rows = self.query(
            "select * from Hint where hunt = $1;",
            &[&hunt_id]);
        for row in &rows {
            hints.push(Hint::from_row(row));
        }
        hints
    }

    pub fn set_hints(&self, hunt_id: i32, hints: &Vec<Hint>) {
        self.execute("delete from Hint where hunt = $1", &[&hunt_id]);
        for hint in hints {
            self.execute("insert into Hint values ($1, $2, $3, $4, $5, $6)",
                         &[&hint.hint, &hint.puzzle_name, &hint.number,
                           &hunt_id, &hint.wave, &hint.key]);
        }
    }


    
    //// Puzzles ////

    pub fn get_released_waves(&self, hunt_id: i32, team: &Option<Team>) -> Vec<ReleasedWave> {
        self.get_waves(hunt_id).into_iter().map(|wave| {
            ReleasedWave {
                puzzles: self.get_released_puzzles(hunt_id, &wave.name, team),
                name: wave.name,
                time: wave.time,
                guesses: wave.guesses,
                released: false // TODO: calculate
            }
        }).collect()
    }

    fn get_released_puzzles(&self, hunt_id: i32, wave: &str, team: &Option<Team>) -> Vec<ReleasedPuzzle> {
        let rows = self.query(
            "select * from Puzzle where hunt = $1 and wave = $2;",
            &[&hunt_id, &wave]);
        let mut puzzles = vec!();
        for row in &rows {
            let puzzle = Puzzle::from_row(row);
            match self.get_wave(hunt_id, &puzzle.wave) {
                None => (),
                Some(wave) => {
                    if wave.is_released() {
                        let solved = match team {
                            None => false,
                            Some(team) => self.is_solved(hunt_id, team.team_id, &puzzle.key)
                        };
                        let answer = if solved { self.get_answer(hunt_id, &puzzle.key) } else { "".to_string() };
                        puzzles.push(ReleasedPuzzle {
                            hints: self.get_released_hints(hunt_id, &puzzle.name),
                            name: puzzle.name,
                            hunt: hunt_id,
                            time: wave.time,
                            wave: puzzle.wave,
                            key: puzzle.key,
                            answer
                        });
                    }
                }
            }
        }
        puzzles
    }

    fn get_released_hints(&self, hunt_id: i32, puzzle_name: &str) -> Vec<Hint> {
        self.query(
            "select * from Hint where hunt = $1 and puzzle_name = $2;",
            &[&hunt_id, &puzzle_name])
            .into_iter()
            .map(|row| Hint::from_row(row))
            .filter(|hint| {
                self.get_wave(hunt_id, &hint.wave).map_or(false, |w| w.is_released())
            })
            .collect()
    }

    pub fn get_released_hint(&self, hunt_id: i32, hint_key: &str) -> Option<Hint> {
        let rows = self.query(
            "select * from Hint where hunt = $1 and key = $2;",
            &[&hunt_id, &hint_key]);
        if rows.len() == 1 {
            Some(Hint::from_row(rows.get(0)))
        } else {
            None
        }
    }

    //// Answer Submission ////
    
    pub fn get_released_puzzle(&self, hunt_id: i32, puzzle_key: &str) -> Option<ReleasedPuzzle> {
        let rows = self.query(
            "select * from Puzzle where hunt = $1 and key = $2",
            &[&hunt_id, &puzzle_key]);
        if rows.len() != 1 {
            return None;
        }
        let puzzle = Puzzle::from_row(rows.get(0));
        match self.get_wave(hunt_id, &puzzle.wave) {
            None => None,
            Some(wave) => {
                if wave.is_released() {
                    Some(ReleasedPuzzle {
                        hints: self.get_released_hints(hunt_id, &puzzle.name),
                        name: puzzle.name,
                        hunt: hunt_id,
                        time: wave.time,
                        wave: puzzle.wave,
                        key: puzzle.key,
                        answer: "".to_string() // innocent lie
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn submit_guess(&self, team: &Team, puzzle: &ReleasedPuzzle, guess: &str) -> Judgement {
        let guess = guess.to_ascii_uppercase();
        // Already solved?
        if let Some(judgement) = self.already_solved(team, puzzle) {
            return judgement;
        }
        // Out of guesses?
        if let Some(judgement) = self.out_of_guesses(team) {
            return judgement;
        }
        // Correct answer?
        let answer = self.get_answer(puzzle.hunt, &puzzle.key);
        if guess.eq_ignore_ascii_case(&answer) {
            self.record_correct_guess(team, puzzle);
            return Judgement {
                guess: guess.to_string(),
                correctness: Correctness::Right
            };
        }
        // Already submitted this answer?
        if self.already_guessed(puzzle, &guess) {
            // Don't subtract a guess.
            return Judgement {
                guess: guess.to_string(),
                correctness: Correctness::AlreadyGuessedThat
            };
        }
        // Wrong!
        self.record_incorrect_guess(team, puzzle, &guess);
        Judgement {
            guess: guess.to_string(),
            correctness: Correctness::Wrong
        }
    }

    pub fn get_answer(&self, hunt_id: i32, puzzle_key: &str) -> String {
        let rows = self.query(
            "select answer from Puzzle where hunt = $1 and key = $2",
            &[&hunt_id, &puzzle_key]);
        if rows.len() != 1 {
            panic!("Puzzle not found {}", puzzle_key);
        }
        rows.get(0).get(0)
    }

    pub fn already_solved(&self, team: &Team, puzzle: &ReleasedPuzzle) -> Option<Judgement> {
        if self.is_solved(puzzle.hunt, team.team_id, &puzzle.key) {
            let answer = self.get_answer(puzzle.hunt, &puzzle.key);
            Some(Judgement {
                guess: answer, // reveal answer
                correctness: Correctness::AlreadySolved
            })
        } else {
            None
        }
    }

    fn is_solved(&self, hunt_id: i32, team_id: i32, puzzle_key: &str) -> bool {
        let rows = self.query(
            "select COUNT(*) from Solve where hunt = $1 and puzzle_key = $2 and team_id = $3",
            &[&hunt_id, &puzzle_key, &team_id]);
        let n: i64 = rows.get(0).get(0);
        n > 0
    }

    fn already_guessed(&self, puzzle: &ReleasedPuzzle, guess: &str) -> bool {
        let rows = self.query(
            "select COUNT(*) from Guess where hunt = $1 and puzzle_key = $2 and guess = $3",
            &[&puzzle.hunt, &puzzle.key, &guess]);
        let n: i64 = rows.get(0).get(0);
        n > 0
    }

    pub fn out_of_guesses(&self, team: &Team) -> Option<Judgement> {
        if team.guesses > 0 {
            None
        } else {
            Some(Judgement {
                guess: "".to_string(), // not used
                correctness: Correctness::OutOfGuesses
            })
        }
    }

    fn record_correct_guess(&self, team: &Team, puzzle: &ReleasedPuzzle) {
        let solve_time: i32 = (Local::now() - puzzle.time).num_seconds() as i32;
        self.execute(
            "insert into Solve values($1, $2, $3, $4, $5)",
            &[&team.team_id, &puzzle.hunt, &puzzle.key, &Utc::now(), &solve_time]);
    }

    fn record_incorrect_guess(&self, team: &Team, puzzle: &ReleasedPuzzle, guess: &str) {
        self.execute(
            "insert into Guess values ($1, $2, $3, $4, $5)",
            &[&team.team_id, &puzzle.hunt, &puzzle.key, &guess, &Utc::now()]);
        self.execute(
            "update Team set guesses = guesses - 1 where hunt = $1 and team_id = $2",
            &[&team.hunt, &team.team_id]);
    }
    

    //// Puzzle/Team Stats ////

    pub fn get_puzzle_stats(&self, hunt_id: i32) -> Vec<PuzzleStats> {
        self.get_waves(hunt_id)
            .into_iter()
            .filter(|wave| wave.is_released())
            .flat_map(|wave| {
                self.get_puzzle_stats_for_wave(hunt_id, &wave.name)
            })
            .collect()
    }

    fn get_puzzle_stats_for_wave(&self, hunt_id: i32, wave: &str) -> Vec<PuzzleStats> {
        let rows = self.query(
            "select key, name from Puzzle where hunt = $1 and wave = $2",
            &[&hunt_id, &wave]);
        rows.into_iter().map(|row| {
            let puzzle_key = row.get(0);
            let puzzle_name = row.get(1);
            let guesses: i64 = self.query(
                "select count(*) from Guess where hunt = $1 and puzzle_key = $2",
                &[&hunt_id, &puzzle_key])
                .get(0).get(0);
            let rows = self.query(
                "select count(*), sum(solve_time) from Solve where hunt = $1 and puzzle_key = $2",
                &[&hunt_id, &puzzle_key]);
            let solves: i64 = rows.get(0).get(0);
            let total_solve_time: Option<i64> = rows.get(0).get(1);
            PuzzleStats {
                wave_name: wave.to_string(),
                puzzle_name,
                puzzle_key,
                guesses: guesses as i32,
                solves: solves as i32,
                total_solve_time: total_solve_time.unwrap_or(0) as i32
            }
        }).collect()
    }
    
    pub fn get_team_stats(&self, hunt_id: i32) -> Vec<TeamStats> {
        let rows = self.query(
            "select team_id, name from Team where hunt = $1",
            &[&hunt_id]);
        rows.into_iter()
            .map(|row| {
                let team_id: i32 = row.get(0);
                let team_name: String = row.get(1);
                let rows = self.query(
                    "select count(*) from Guess where hunt = $1 and team_id = $2",
                    &[&hunt_id, &team_id]);
                let guesses: i64 = rows.get(0).get(0);
                let rows = self.query(
                    "select count(*), sum(solve_time) from Solve where hunt = $1 and team_id = $2",
                    &[&hunt_id, &team_id]);
                let row = rows.get(0);
                let solves: i64 = row.get(0);
                let total_solve_time: Option<i64> = row.get(1);
                TeamStats {
                    team_name,
                    guesses: guesses as i32,
                    solves: solves as i32,
                    total_solve_time: total_solve_time.unwrap_or(0) as i32
                }
            })
            .collect()
    }

    
    //// Teams ////

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

    fn fill_team_members(&self, team: &mut Team) {
        let rows = self.query(
            "select * from Member where team_id = $1",
            &[&team.team_id]);
        let members = rows.iter().map(|row| Member::from_row(row)).collect();
        team.members = members;
    }

    pub fn create_team(&self, hunt_id: i32, form: &CreateTeam) -> Result<Team, String> {
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
            "insert into Team values (default, $1, $2, $3, $4) returning team_id",
            &[&hunt_id, &form.password, &form.name, &hunt.init_guesses]).get(0).get(0);
        for member in &form.members {
            self.execute(
                "insert into Member values ($1, $2, $3, $4)",
                &[&team_id, &hunt_id, &member.name, &member.email]);
        }

        // Return newly registred team
        match self.get_team(hunt_id, &form.name) {
            None => Err("Failed to find team.".to_string()),
            Some(team) => Ok(team)
        }
    }

    fn team_exists(&self, hunt_id: i32, name: &str) -> bool {
        let rows = self.query(
            "select * from Team where hunt = $1 and name = $2;",
            &[&hunt_id, &name]);
        rows.len() >= 1
    }
    
    pub fn update_team(&self, hunt_id: i32, form: &UpdateTeam) -> Result<Team, String> {
        // Validate
        let team = match self.get_team(hunt_id, &form.name) {
            None => return Err("Team does not exist, or password does not match.".to_string()),
            Some(team) => team
        };
        
        // Update
        self.execute(
            "delete from Member where team_id = $1 and hunt = $2",
            &[&team.team_id, &hunt_id]);
        for member in &form.members {
            self.execute(
                "insert into Member values ($1, $2, $3, $4)",
                &[&team.team_id, &hunt_id, &member.name, &member.email]);
        }

        // Return updated team
        match self.get_team(hunt_id, &form.name) {
            None => Err("Failed to find team.".to_string()),
            Some(team) => Ok(team)
        }
    }
}
