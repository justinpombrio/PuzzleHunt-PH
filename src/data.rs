use chrono::{Local, Utc, DateTime};
use mustache::{MapBuilder, VecBuilder, Data};
use postgres::rows::Row;


pub trait DBTable {
    fn from_row(row: Row) -> Self;
    fn drop_query() -> &'static str;
    fn init_query() -> &'static str;
    fn test_init_query() -> &'static str;
}

pub trait TemplateData {
    fn name() -> &'static str;
    fn names() -> &'static str;
    fn to_data(&self, builder: MapBuilder) -> MapBuilder;
}


pub fn build_data(items: Vec<&AddToData>) -> Data {
    let mut builder = MapBuilder::new();
    for item in &items {
        builder = item.add_to_data(builder);
    }
    builder.build()
}


pub trait AddToData {
    fn add_to_data(&self, builder: MapBuilder) -> MapBuilder;
}

impl<C : TemplateData> AddToData for C {
    fn add_to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder.insert_map(Self::name(), |m| self.to_data(m))
    }
}

impl<C : TemplateData> AddToData for Vec<C> {
    fn add_to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder.insert_vec(C::names(), |b| vec_to_data(self, b))
    }
}

fn vec_to_data<C : TemplateData>(items: &Vec<C>, builder: VecBuilder) -> VecBuilder {
    let mut builder = builder;
    for item in items {
        builder = builder.push_map(|map| item.to_data(map))
    }
    builder
}


////// Site //////

#[derive(Debug, Clone)]
pub struct Site {
    pub owner: String,
    pub secret: String
}

impl TemplateData for Site {
    fn name()  -> &'static str { "site" }
    fn names() -> &'static str { "sites" }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("owner",  self.owner.clone())
            .insert_str("secret", self.secret.clone())
    }
}

impl DBTable for Site {
    fn from_row(row: Row) -> Site {
        Site{
            owner:  row.get(0),
            secret: row.get(1)
        }
    }

    fn drop_query() -> &'static str {
        "drop table if exists Site;"
    }

    fn init_query() -> &'static str {
"create table Site (
  owner varchar NOT NULL,
  secret varchar NOT NULL
);"
    }

    fn test_init_query() -> &'static str {
"insert into Site (owner, secret)
values ('me', 'secret');"
    }
}



////// Hunts //////

#[derive(Debug, Clone)]
pub struct Hunt {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub team_size: i32,
    pub init_guesses: i32,
    pub password: String,
    pub closed: bool,
    pub visible: bool
}

impl TemplateData for Hunt {
    fn name()  -> &'static str { "hunt" }
    fn names() -> &'static str { "hunts" }
    
    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("id",           format!("{}", self.id))
            .insert_str("name",         self.name.clone())
            .insert_str("key",          self.key.clone())
            .insert_str("team_size",    format!("{}", self.team_size))
            .insert_str("init_guesses", format!("{}", self.init_guesses))
            .insert_bool("closed",      self.closed)
            .insert_bool("visible",     self.visible)
    }
}

impl DBTable for Hunt {
    fn from_row(row: Row) -> Hunt {
        Hunt{
            id:           row.get(0),
            name:         row.get(1),
            key:          row.get(2),
            team_size:    row.get(3),
            init_guesses: row.get(4),
            password:     row.get(5),
            closed:       row.get(6),
            visible:      row.get(7)
        }
    }

    fn drop_query() -> &'static str {
        "drop table if exists Hunt;"
    }

    fn init_query() -> &'static str {
"create table Hunt (
  id serial primary key NOT NULL,
  name varchar NOT NULL,
  key varchar NOT NULL,
  team_size int NOT NULL,
  init_guesses int NOT NULL,
  password varchar NOT NULL,
  closed boolean NOT NULL,
  visible boolean NOT NULL
);"
    }

    fn test_init_query() -> &'static str {
"insert into Hunt (name, key, team_size, init_guesses, password, closed, visible)
values ('Best Hunt Ever', 'besthuntever', 4, 100, 'pass', true, true);"
    }
}


////// Waves //////


#[derive(Debug, Clone)]
pub struct ReleasedWave {
    pub name: String,
    pub time: DateTime<Local>,
    pub guesses: i32,
    pub released: bool,
    pub puzzles: Vec<ReleasedPuzzle>
}

impl TemplateData for ReleasedWave {
    fn name()  -> &'static str { "wave" }
    fn names() -> &'static str { "waves" }
    
    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("name",      self.name.clone())
            .insert_str("time",      self.time.to_rfc3339())
            .insert_str("guesses",   format!("{}", self.guesses))
            .insert_bool("released", self.released)
            .insert_vec("puzzles",   |b| vec_to_data(&self.puzzles, b))
    }
}

#[derive(Debug, Clone)]
pub struct Wave {
    pub name: String,
    pub time: DateTime<Local>,
    pub guesses: i32
}

impl Wave {
    pub fn is_released(&self) -> bool {
        Local::now() > self.time
    }
}

impl TemplateData for Wave {
    fn name()  -> &'static str { "wave" }
    fn names() -> &'static str { "waves" }
    
    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("name",      self.name.clone())
            .insert_str("time",      self.time.to_rfc3339())
            .insert_str("guesses",   format!("{}", self.guesses))
    }
}

impl DBTable for Wave {
    fn from_row(row: Row) -> Wave {
        let time: DateTime<Utc> = row.get(2);
        Wave{
            name:     row.get(0),
            time:     time.with_timezone(&Local),
            guesses:  row.get(3)
        }
    }

    fn drop_query() -> &'static str {
        "drop table if exists Wave;"
    }

    fn init_query() -> &'static str {
"create table Wave (
  name varchar NOT NULL,
  hunt int NOT NULL,
  time timestamp with time zone NOT NULL,
  guesses int NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Wave (name, hunt, time, guesses)
values ('Wave One', 1, '2004-10-19 10:23:54', 10);"
    }
}


////// Puzzles //////

#[derive(Debug, Clone)]
pub struct ReleasedPuzzle {
    pub name: String,
    pub hunt: i32,
    pub wave: String,
    pub time: DateTime<Local>,
    pub key: String,
    pub hints: Vec<Hint>
}

impl TemplateData for ReleasedPuzzle {
    fn name()  -> &'static str { "puzzle" }
    fn names() -> &'static str { "puzzles" }
    
    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("name",   self.name.clone())
            .insert_str("wave",   self.wave.clone())
            .insert_str("key",    self.key.clone())
            .insert_vec("hints",  |b| vec_to_data(&self.hints, b))
    }
}

// TODO: Let's just make name the primary key
#[derive(Debug, Clone)]
pub struct Puzzle {
    pub name: String,
    pub hunt: i32,
    pub answer: String,
    pub wave: String,
    pub key: String
}

impl TemplateData for Puzzle {
    fn name()  -> &'static str { "puzzle" }
    fn names() -> &'static str { "puzzles" }
    
    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("name",      self.name.clone())
            .insert_str("hunt",      format!("{}", self.hunt))
            .insert_str("answer",    self.answer.clone())
            .insert_str("wave",      self.wave.clone())
            .insert_str("key",       self.key.clone())
    }
}

impl DBTable for Puzzle {
    fn from_row(row: Row) -> Puzzle {
        Puzzle{
            name:           row.get(0),
            hunt:           row.get(1),
            answer:         row.get(2),
            wave:           row.get(3),
            key:            row.get(4)
        }
    }

    fn drop_query() -> &'static str {
        "drop table if exists Puzzle;"
    }

    fn init_query() -> &'static str {
"create table Puzzle (
  name varchar primary key NOT NULL,
  hunt int NOT NULL,
  answer varchar NOT NULL,
  wave varchar NOT NULL,
  key varchar NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Puzzle (name, hunt, answer, wave, key)
values ('Puzzle Two', 1, 'answer2', 'Wave One', 'QQQ'),
       ('Puzzle One', 1, 'answer1', 'Wave One', 'PPP'),
       ('Puzzle Three', 1, 'answer3', 'Wave One', 'RRR');"
    }
}


////// Hints //////

#[derive(Debug, Clone)]
pub struct Hint {
    pub hint: String,
    pub puzzle_name: String,
    pub number: i32,
    pub hunt: i32,
    pub wave: String,
    pub key: String
}

impl TemplateData for Hint {
    fn name()  -> &'static str { "hint" }
    fn names() -> &'static str { "hints" }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("hint",      self.hint.clone())
            .insert_str("puzzle_name",self.puzzle_name.clone())
            .insert_str("number",    format!("{}", self.number))
            .insert_str("wave",      self.wave.clone())
            .insert_str("key",       self.key.clone())
    }
}

impl DBTable for Hint {
    fn from_row(row: Row) -> Hint {
        Hint{
            hint:       row.get(0),
            puzzle_name:row.get(1),
            number:     row.get(2),
            hunt:       row.get(3),
            wave:       row.get(4),
            key:        row.get(5)
        }
    }

    fn drop_query() -> &'static str {
        "drop table if exists Hint;"
    }

    fn init_query() -> &'static str {
"create table Hint (
  hint varchar NOT NULL,
  puzzle_name varchar NOT NULL,
  number int NOT NULL,
  hunt int NOT NULL,
  wave varchar NOT NULL,
  key varchar NOT NULL,
  primary key (hunt, puzzle_name)
);
"        
    }

    fn test_init_query() -> &'static str {
"insert into Hint (hint, puzzle_name, number, hunt, wave, key)
values ('The answer is \"answer\".', 'Puzzle One', 1, 1, 'Wave One', 'HHH');"
    }
}


////// Teams //////

#[derive(Debug, Clone)]
pub struct Team {
    pub team_id: i32,
    pub hunt: i32,
    pub password: String,
    pub name: String,
    pub guesses: i32,
    pub members: Vec<Member>
}

impl TemplateData for Team {
    fn name()  -> &'static str { "team" }
    fn names() -> &'static str { "teams" }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("team_id",  format!("{}", self.team_id))
            .insert_str("hunt",     format!("{}", self.hunt))
            .insert_str("password", self.password.clone())
            .insert_str("name",     self.name.clone())
            .insert_str("guesses",  format!("{}", self.guesses))
            .insert_vec("members",  |b| vec_to_data(&self.members, b))
    }
}

impl DBTable for Team {
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
    
    fn drop_query() -> &'static str {
        "drop table if exists Team;"
    }

    fn init_query() -> &'static str {
"create table Team (
  team_id serial primary key NOT NULL,
  hunt int NOT NULL,
  password varchar NOT NULL,
  name varchar NOT NULL,
  guesses int NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Team (hunt, password, name, guesses)
values (1, 'pass', 'BestTeamEver', 5),
(1, 'pass', 'SecondBestTeam', 99);"
    }
}


////// Members //////

#[derive(Debug, Clone)]
pub struct Member {
    pub team_id: i32,
    pub hunt: i32,
    pub name: String,
    pub email: String
}

impl TemplateData for Member {
    fn name()  -> &'static str { "member" }
    fn names() -> &'static str { "members" }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("team_id", format!("{}", self.team_id))
            .insert_str("hunt", format!("{}", self.hunt))
            .insert_str("name", self.name.clone())
            .insert_str("email", self.email.clone())
    }
}

impl DBTable for Member {
    fn from_row(row: Row) -> Member {
        Member{
            team_id: row.get(0),
            hunt:    row.get(1),
            name:    row.get(2),
            email:   row.get(3)
        }
    }

    fn drop_query() -> &'static str {
        "drop table if exists Member;"
    }

    fn init_query() -> &'static str {
"create table Member (
  team_id int NOT NULL,
  hunt int NOT NULL,
  name varchar NOT NULL,
  email varchar NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Member (team_id, hunt, name, email)
values (1, 1, 'BestPersonEver', 'person@email.com');"
    }
}


////// Guesses //////

#[derive(Debug, Clone)]
pub struct Guess {
    pub team_id: i32,
    pub hunt: i32,
    pub puzzle_key: String,
    pub guess: String,
    pub time: DateTime<Utc>
}

impl TemplateData for Guess {
    fn name()  -> &'static str { "guess" }
    fn names() -> &'static str { "guesss" }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("team_id", format!("{}", self.team_id))
            .insert_str("hunt", format!("{}", self.hunt))
            .insert_str("puzzle_key", self.puzzle_key.clone())
            .insert_str("guess", self.guess.clone())
            .insert_str("time", format!("{}", self.time))
    }
}

impl Guess {
    pub fn index_query() -> &'static str {
        "create index guess_index on Guess (hunt, team_id, puzzle_key);"
    }
}

impl DBTable for Guess {
    fn from_row(row: Row) -> Guess {
        Guess{
            team_id:    row.get(0),
            hunt:       row.get(1),
            puzzle_key: row.get(2),
            guess:      row.get(3),
            time:       row.get(4)
        }
    }

    fn drop_query() -> &'static str {
        "drop table if exists Guess;"
    }

    fn init_query() -> &'static str {
"create table Guess (
  team_id int NOT NULL,
  hunt int NOT NULL,
  puzzle_key varchar NOT NULL,
  guess varchar NOT NULL,
  time timestamp with time zone NOT NULL
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Guess (team_id, hunt, puzzle_key, guess, time)
values (1, 1, 'PPP', 'answer?', '2004-10-19 10:23:54');"
    }
}


#[derive(Debug, Clone)]
pub struct Solve {
    pub team_id: i32,
    pub hunt: i32,
    pub puzzle_key: String,
    pub solved_at: DateTime<Utc>,
    pub solve_time: i32, // in seconds
}

impl DBTable for Solve {
    fn from_row(row: Row) -> Solve {
        Solve {
            team_id:    row.get(0),
            hunt:       row.get(1),
            puzzle_key: row.get(2),
            solved_at:  row.get(3),
            solve_time: row.get(4),
        }
    }

    fn drop_query() -> &'static str {
        "drop table if exists Solve;"
    }

    fn init_query() -> &'static str {
"create table Solve (
  team_id int NOT NULL,
  hunt int NOT NULL,
  puzzle_key varchar NOT NULL,
  solved_at timestamp with time zone NOT NULL,
  solve_time int NOT NULL,
  primary key (hunt, team_id, puzzle_key)
);
"
    }

    fn test_init_query() -> &'static str {
"insert into Solve (team_id, hunt, puzzle_key, solved_at, solve_time)
values (1, 1, 'PPP', '2004-10-19 10:23:54', 385),
       (2, 1, 'PPP', '2004-10-19 10:23:55', 386);"
    }
}


////// Answer Submission //////

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Correctness {
    Right,
    Wrong,
    AlreadySolved,
    AlreadyGuessedThat,
    OutOfGuesses
}

#[derive(Debug, Clone)]
pub struct Judgement {
    pub puzzle_name: String,
    pub guess: String,
    pub correctness: Correctness,
    pub guesses_remaining: i32
}

impl TemplateData for Judgement {
    fn name() -> &'static str {
        "judgement"
    }
    fn names() -> &'static str {
        "judgements"
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        builder
            .insert_str("puzzle_name", self.puzzle_name.clone())
            .insert_str("guess", self.guess.clone())
            .insert_bool("is_right", self.correctness == Correctness::Right)
            .insert_bool("is_wrong", self.correctness == Correctness::Wrong)
            .insert_bool("is_already_solved", self.correctness == Correctness::AlreadySolved)
            .insert_bool("is_already_guessed", self.correctness == Correctness::AlreadyGuessedThat)
            .insert_bool("is_out_of_guesses", self.correctness == Correctness::OutOfGuesses)
            .insert_str("guesses_remaining", format!("{}", self.guesses_remaining))
    }
}




////// Stats //////


#[derive(Debug, Clone)]
pub struct TeamStats {
    pub team_name: String,
    pub guesses: i32,
    pub solves: i32,
    pub total_solve_time: i32, // in seconds
}

impl TemplateData for TeamStats {
    fn name() -> &'static str {
        "stat"
    }
    fn names() -> &'static str {
        "stats"
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        let avg_solve_time = if self.solves == 0 {
            "-".to_string()
        } else {
            format!("{} mins", self.total_solve_time / self.solves / 60)
        };
        builder
            .insert_str("team", self.team_name.clone())
            .insert_str("guesses", format!("{}", self.guesses))
            .insert_str("avg_solve_time", avg_solve_time)
    }
}


#[derive(Debug, Clone)]
pub struct PuzzleStats {
    pub wave_name: String,
    pub puzzle_name: String,
    pub puzzle_key: String,
    pub guesses: i32,
    pub solves: i32,
    pub total_solve_time: i32, // in seconds
}

impl TemplateData for PuzzleStats {
    fn name() -> &'static str {
        "stat"
    }
    fn names() -> &'static str {
        "stats"
    }

    fn to_data(&self, builder: MapBuilder) -> MapBuilder {
        let avg_solve_time = if self.solves == 0 {
            "-".to_string()
        } else {
            format!("{} mins", self.total_solve_time / self.solves / 60)
        };
        builder
            .insert_str("wave_name", self.wave_name.clone())
            .insert_str("puzzle_name", self.puzzle_name.clone())
            .insert_str("puzzle_key", self.puzzle_key.clone())
            .insert_str("guesses", format!("{}", self.guesses))
            .insert_str("solves", format!("{}", self.solves))
            .insert_str("avg_solve_time", avg_solve_time)
    }
}
