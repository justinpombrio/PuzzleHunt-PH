use chrono::{UTC, DateTime};
use mustache::{MapBuilder};
use postgres::rows::Row;

pub trait Convert {
    fn from_row(row: Row) -> Self;
    fn to_data(&self, map: MapBuilder) -> MapBuilder;
    fn drop_query() -> &'static str;
    fn init_query() -> &'static str;
    fn test_init_query() -> &'static str;
}


////// Hunts //////

#[derive(Debug)]
pub struct Hunt {
    pub hunt_id: i32,
    pub name: String,
    pub key: String,
    pub team_size: i32,
    pub init_guesses: i32,
    pub password: String,
    pub secret_key: String,
    pub closed: bool
}

impl Convert for Hunt {
    fn from_row(row: Row) -> Hunt {
        Hunt{
            hunt_id:      row.get(0),
            name:         row.get(1),
            key:          row.get(2),
            team_size:    row.get(3),
            init_guesses: row.get(4),
            password:     row.get(5),
            secret_key:   row.get(6),
            closed:       row.get(7)
        }
    }

    fn to_data(&self, map: MapBuilder) -> MapBuilder {
        map
            .insert_str("huntID", self.hunt_id)
            .insert_str("name", &self.name)
            .insert_str("key", &self.key)
            .insert_str("teamSize", self.team_size)
            .insert_str("initGuesses", self.init_guesses)
            .insert_str("closed", self.closed)
    }

    fn drop_query() -> &'static str {
        "drop table if exists Hunt;"
    }

    fn init_query() -> &'static str {
"create table Hunt (
  huntID serial primary key NOT NULL,
  name varchar NOT NULL,
  key varchar NOT NULL,
  teamSize int NOT NULL,
  initGuesses int NOT NULL,
  password varchar NOT NULL,
  secretKey varchar NOT NULL,
  closed boolean NOT NULL
);"
    }

    fn test_init_query() -> &'static str {
"insert into Hunt (name, key, teamSize, initGuesses, password, secretKey, closed)
values ('Best Hunt Ever', 'besthuntever', 4, 100, 'pass', 'secret', true);"
    }
}


////// Waves //////


#[derive(Debug)]
pub struct Wave {
    pub name: String,
    pub hunt: i32,
    pub time: DateTime<UTC>,
    pub guesses: i32,
    pub released: bool,
    pub puzzles: Vec<Puzzle>
}

impl Convert for Wave {
    fn from_row(row: Row) -> Wave {
        Wave{
            name:     row.get(0),
            hunt:     row.get(1),
            time:     row.get(2),
            guesses:  row.get(3),
            released: row.get(4),
            puzzles:  vec!()
        }
    }

    fn to_data(&self, map: MapBuilder) -> MapBuilder {
        map
            .insert_str("name", &self.name)
            .insert_str("hunt", self.hunt)
            .insert_str("time", &self.time)
            .insert_str("guesses", self.guesses)
            .insert_str("released", self.released)
            .insert_vec("puzzles", |mut puzzles| {
                for puzzle in &self.puzzles {
                    puzzles = puzzles.push_map(|p| puzzle.to_data(p))
                }
                puzzles
            })
    }

    fn drop_query() -> &'static str {
        "drop table if exists Wave;"
    }

    fn init_query() -> &'static str {
"create table Wave (
  name varchar NOT NULL,
  hunt int NOT NULL,
  time timestamp with time zone NOT NULL,
  guesses int NOT NULL,
  released boolean NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Wave (name, hunt, time, guesses, released)
values ('Wave One', 1, '2004-10-19 10:23:54', 10, true);"
    }
}


////// Puzzles //////

#[derive(Debug)]
pub struct Puzzle {
    pub name: String,
    pub number: String,
    pub hunt: i32,
    pub base_points: i32,
    pub current_points: i32,
    pub answer: String,
    pub wave: String,
    pub key: String,
    pub released: bool,
    pub hints: Vec<Hint>
}

impl Convert for Puzzle {
    fn from_row(row: Row) -> Puzzle {
        Puzzle{
            name:           row.get(0),
            number:         row.get(1),
            hunt:           row.get(2),
            base_points:    row.get(3),
            current_points: row.get(4),
            answer:         row.get(5),
            wave:           row.get(6),
            key:            row.get(7),
            released:       row.get(8),
            hints:          vec!()
        }
    }

    fn to_data(&self, map: MapBuilder) -> MapBuilder {
        map
            .insert_str("name", &self.name)
            .insert_str("number", &self.number)
            .insert_str("hunt", self.hunt)
            .insert_str("basePoints", self.base_points)
            .insert_str("currentPoints", self.current_points)
            .insert_str("wave", &self.wave)
            .insert_str("key", &self.key)
            .insert_str("released", self.released)
            .insert_vec("hints", |mut hints| {
                for hint in &self.hints {
                    hints = hints.push_map(|h| hint.to_data(h));
                }
                hints
            })
    }

    fn drop_query() -> &'static str {
        "drop table if exists Puzzle;"
    }

    fn init_query() -> &'static str {
"create table Puzzle (
  name varchar primary key NOT NULL,
  number varchar NOT NULL,
  hunt int NOT NULL,
  basePoints int NOT NULL,
  currentPoints int NOT NULL,
  answer varchar NOT NULL,
  wave varchar NOT NULL,
  key varchar NOT NULL,
  released boolean NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Puzzle (name, number, hunt, basePoints, currentPoints, answer, wave, key, released)
values ('Puzzle One', '#1', 1, 2, 1, 'answer', 'Wave One', 'PPP', true);"
    }
}


////// Hints //////

#[derive(Debug)]
pub struct Hint {
    pub puzzle: String,
    pub number: i32,
    pub hunt: i32,
    pub penalty: i32,
    pub wave: String,
    pub key: String,
    pub released: bool
}

impl Convert for Hint {
    fn from_row(row: Row) -> Hint {
        Hint{
            puzzle:   row.get(0),
            number:   row.get(1),
            hunt:     row.get(2),
            penalty:  row.get(3),
            wave:     row.get(4),
            key:      row.get(5),
            released: row.get(6)
        }
    }

    fn to_data(&self, map: MapBuilder) -> MapBuilder {
        map
            .insert_str("puzzle", &self.puzzle)
            .insert_str("number", self.number)
            .insert_str("hunt", self.hunt)
            .insert_str("penalty", self.penalty)
            .insert_str("wave", &self.wave)
            .insert_str("key", &self.key)
            .insert_str("released", self.released)
    }

    fn drop_query() -> &'static str {
        "drop table if exists Hint;"
    }

    fn init_query() -> &'static str {
"create table Hint (
  puzzle varchar NOT NULL,
  number int NOT NULL,
  hunt int NOT NULL,
  penalty int NOT NULL,
  wave varchar NOT NULL,
  key varchar NOT NULL,
  released boolean NOT NULL
);
"        
    }

    fn test_init_query() -> &'static str {
"insert into Hint (puzzle, number, hunt, penalty, wave, key, released)
values ('Puzzle One', 1, 1, 1, 'Wave One', 'HHH', true);"
    }
}


////// Teams //////

#[derive(Debug)]
pub struct Team {
    pub team_id: i32,
    pub hunt: i32,
    pub password: String,
    pub name: String,
    pub guesses: i32,
    pub members: Vec<Member>
}

impl Convert for Team {
    fn from_row(row: Row) -> Team {
        Team{
            team_id:  row.get(0),
            hunt:     row.get(1),
            password: row.get(2),
            name:     row.get(3),
            guesses:  row.get(4),
            members:  vec!()
        }
    }

    fn to_data(&self, map: MapBuilder) -> MapBuilder {
        map
            .insert_str("teamID",   self.team_id)
            .insert_str("hunt",     self.hunt)
            .insert_str("password", &self.password)
            .insert_str("name",     &self.name)
            .insert_str("guesses",  self.guesses)
            .insert_vec("members", |mut members| {
                for member in &self.members {
                    members = members.push_map(|m| member.to_data(m))
                }
                members
            })
    }
    
    fn drop_query() -> &'static str {
        "drop table if exists Team;"
    }

    fn init_query() -> &'static str {
"create table Team (
  teamID serial primary key NOT NULL,
  hunt int NOT NULL,
  password varchar NOT NULL,
  name varchar NOT NULL,
  guesses int NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Team (hunt, password, name, guesses)
values (1, 'pass', 'BestTeamEver', 50);"
    }
}


////// Members //////

#[derive(Debug)]
pub struct Member {
    pub team_id: i32,
    pub hunt: i32,
    pub name: String,
    pub email: String
}

impl Convert for Member {
    fn from_row(row: Row) -> Member {
        Member{
            team_id: row.get(0),
            hunt:    row.get(1),
            name:    row.get(2),
            email:   row.get(3)
        }
    }

    fn to_data(&self, map: MapBuilder) -> MapBuilder {
        map
            .insert_str("teamID", self.team_id)
            .insert_str("hunt", self.hunt)
            .insert_str("name", &self.name)
            .insert_str("email", &self.email)
    }

    fn drop_query() -> &'static str {
        "drop table if exists Member;"
    }

    fn init_query() -> &'static str {
"create table Member (
  teamID int NOT NULL,
  hunt int NOT NULL,
  name varchar NOT NULL,
  email varchar NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Member (teamID, hunt, name, email)
values (1, 1, 'BestPersonEver', 'person@email.com');"
    }
}


////// Guesses //////

#[derive(Debug)]
pub struct Guess {
    pub team_id: i32,
    pub hunt: i32,
    pub puzzle: String,
    pub guess: String,
    pub time: DateTime<UTC>
}

impl Convert for Guess {
    fn from_row(row: Row) -> Guess {
        Guess{
            team_id: row.get(0),
            hunt:    row.get(1),
            puzzle:  row.get(2),
            guess:   row.get(3),
            time:    row.get(4)
        }
    }

    fn to_data(&self, map: MapBuilder) -> MapBuilder {
        map
            .insert_str("teamID", self.team_id)
            .insert_str("hunt", self.hunt)
            .insert_str("puzzle", &self.puzzle)
            .insert_str("guess", &self.guess)
            .insert_str("time", &self.time)
    }

    fn drop_query() -> &'static str {
        "drop table if exists Guess;"
    }

    fn init_query() -> &'static str {
"create table Guess (
  teamID int NOT NULL,
  hunt int NOT NULL,
  puzzle varchar NOT NULL,
  guess varchar NOT NULL,
  time timestamp with time zone NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Guess (teamID, hunt, puzzle, guess, time)
values (1, 1, 'Puzzle One', 'answer?', '2004-10-19 10:23:54');"
    }
}


////// Solves //////

pub struct Solve {
    pub team_id: i32,
    pub hunt: i32,
    pub puzzle: String,
    pub time: DateTime<UTC>
}

impl Convert for Solve {
    fn from_row(row: Row) -> Solve {
        Solve{
            team_id: row.get(0),
            hunt:    row.get(1),
            puzzle:  row.get(2),
            time:    row.get(3)
        }
    }

    fn to_data(&self, map: MapBuilder) -> MapBuilder {
        map
            .insert_str("teamID", self.team_id)
            .insert_str("hunt", self.hunt)
            .insert_str("puzzle", &self.puzzle)
            .insert_str("time", &self.time)
    }
    
    fn drop_query() -> &'static str {
        "drop table if exists Solve;"
    }

    fn init_query() -> &'static str {
"create table Solve (
  teamID int NOT NULL,
  hunt int NOT NULL,
  puzzle varchar NOT NULL,
  time timestamp with time zone NOT NULL,
  primary key (teamID, puzzle)
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Solve (teamID, hunt, puzzle, time)
values (1, 1, 'Puzzle One', '2004-10-19 10:23:54');"
    }
}


////// Stats //////

#[derive(Debug)]
pub struct Stats {
    pub team_id: i32,
    pub hunt: i32,
    pub puzzle: String,
    pub score: i32,
    pub solve_time: i32,
    pub guesses: i32
}

impl Convert for Stats {
    fn from_row(row: Row) -> Stats {
        Stats{
            team_id:    row.get(0),
            hunt:       row.get(1),
            puzzle:     row.get(2),
            score:      row.get(3),
            solve_time: row.get(4),
            guesses:    row.get(5)
        }
    }

    fn to_data(&self, map: MapBuilder) -> MapBuilder {
        map
            .insert_str("teamID", self.team_id)
            .insert_str("hunt", self.hunt)
            .insert_str("puzzle", &self.puzzle)
            .insert_str("score", self.score)
            .insert_str("solveTime", self.solve_time)
            .insert_str("guesses", self.guesses)
    }

    fn drop_query() -> &'static str {
        "drop table if exists Stats;"
    }

    fn init_query() -> &'static str {
"create table Stats (
  teamID int NOT NULL,
  hunt int NOT NULL,
  puzzle varchar NOT NULL,
  score int NOT NULL,
  solveTime int,
  guesses int NOT NULL,
  primary key (teamID, puzzle)
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Stats (teamId, hunt, puzzle, score, solveTime, guesses)
values (1, 1, 'Puzzle One', 10, 385, 50);"
    }
}
