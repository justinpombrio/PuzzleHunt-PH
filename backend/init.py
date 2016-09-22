import psycopg2
from sys import exit

try:
    conn = psycopg2.connect("dbname='crums' user='postgres' host='localhost' password='password'")
except:
    print "Can't connect to database!"
    exit(1)

if raw_input("Do you really want to wipe the database? (yes/[no]) ") != "yes":
        exit(0)
if raw_input("Are you sure? (yes/[no]) ") != "yes":
        exit(0)

c = conn.cursor()

string = """
drop table if exists Hunt;
create table Hunt (
  name varchar NOT NULL,
  teamSize smallint NOT NULL,
  initGuesses int NOT NULL,
  password varchar NOT NULL,
  secretKey varchar NOT NULL,
  closed boolean NOT NULL
);

drop table if exists Team;
create table Team (
  teamID serial primary key NOT NULL,
  password varchar NOT NULL,
  name varchar NOT NULL,
  guesses int NOT NULL
);

drop table if exists Member;
create table Member (
  teamID smallint NOT NULL,
  name varchar NOT NULL,
  email varchar NOT NULL
);

drop table if exists Puzzle;
create table Puzzle (
  name varchar primary key NOT NULL,
  number varchar NOT NULL,
  basePoints int NOT NULL,
  currentPoints int NOT NULL,
  answer varchar NOT NULL,
  wave varchar NOT NULL,
  key varchar NOT NULL,
  released boolean NOT NULL
);

drop table if exists Hint;
create table Hint (
  puzzle varchar NOT NULL,
  number smallint NOT NULL,
  penalty int NOT NULL,
  wave varchar NOT NULL,
  key varchar NOT NULL,
  released boolean NOT NULL
);

drop table if exists Wave;
create table Wave (
  name varchar NOT NULL,
  time timestamp NOT NULL,
  guesses int NOT NULL,
  released boolean NOT NULL
);

drop table if exists Guess;
create table Guess (
  teamID smallint NOT NULL,
  puzzle varchar NOT NULL,
  guess varchar NOT NULL,
  time timestamp NOT NULL
);

drop table if exists Solve;
create table Solve (
  teamID smallint NOT NULL,
  puzzle varchar NOT NULL,
  time timestamp NOT NULL,
  primary key (teamID, puzzle)
);

drop table if exists Stats;
create table Stats (
  teamID smallint NOT NULL,
  puzzle varchar NOT NULL,
  score int NOT NULL,
  solveTime int,
  guesses int NOT NULL,
  primary key (teamID, puzzle)
);"""

c.execute(string)
conn.commit()
c.close()
conn.close()

print "RESET"
