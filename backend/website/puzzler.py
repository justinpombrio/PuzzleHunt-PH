from flask import Blueprint, request
from helpers import *
from website import db
import re
import bcrypt
import datetime

puzzler_api = Blueprint('puzzler_api', __name__)

c = db.cursor()
c.execute("SELECT teamSize, initGuesses FROM Hunt")
TEAM_SIZE, INIT_GUESSES = c.fetchone()
email_regex = r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)"
now = datetime.datetime.now

@puzzler_api.route("/registerTeam", methods=['POST'])
def registerTeam():
    releaseWaves()
    # Retrieve data
    fail, content = parseJson(request, {"name": unicode, "password": unicode, "members": [{"name": unicode, "email": unicode}]})
    if fail:
        return content
    team_name = content["name"]
    password = content["password"].encode('UTF_8')
    members = content["members"]
    c = db.cursor()

    # Team name too long
    if tooLong(team_name, "team_name"):
        return abortMessage("Team name too long", c)

    # No team members given
    if not members:
        return abortMessage("Team must have at least one member", c)

    # Too many team members
    if len(members) > TEAM_SIZE:
        return abortMessage("There are too many people on your team", c)

    # If team exists, abort
    c.execute("SELECT teamID FROM Team WHERE name = %s", (team_name,));
    if c.fetchone():
        return abortMessage("Team '%s' already exists" % team_name, c)

    # Add new team, get teamID to add members
    hashed = bcrypt.hashpw(password, bcrypt.gensalt())
    c.execute("INSERT INTO Team VALUES (DEFAULT, %s, %s, %s) RETURNING teamID", (hashed, team_name, INIT_GUESSES))
    teamID = c.fetchone()[0]

    # Add each team member
    for member in members:
        member_name = member["name"]
        email = member["email"]
        # Bad email address
        if tooLong(member_name, "member_name"):
            return abortMessage("Member name too long", c, db)
        elif not re.search(email_regex, email) or tooLong(email, "email"):
            return abortMessage("Invalid email address", c, db)

        c.execute("INSERT INTO Member VALUES (%s, %s, %s)", (teamID, member_name, email))

    return success({}, c, db)


@puzzler_api.route("/viewTeam", methods=['POST'])
def viewTeam():
    releaseWaves()
    fail, content = parseJson(request, {"name": unicode})
    if fail:
        return content
    team_name = content["name"]
    c = db.cursor()

    # Get teamID, check exists
    c.execute("SELECT teamID FROM Team WHERE name = %s", (team_name,))
    teamID = c.fetchone()
    if teamID == None:
        return abortMessage("Team '%s' does not exist" % team_name, c)

    # Get all team members
    c.execute("SELECT name FROM Member WHERE teamID = %s", (teamID,))
    members = [{"name": name[0]} for name in c.fetchall()]

    return success({"members": members}, c)


@puzzler_api.route("/changePassword", methods=['POST'])
def changePassword():
    releaseWaves()
    fail, content = parseJson(request, {"name": unicode, "password": unicode, "newPassword": unicode})
    if fail:
        return content
    team_name = content["name"]
    password = content["password"].encode('UTF_8')
    newPassword = content["newPassword"]
    c = db.cursor()

    auth, teamID = authorized(team_name, password, c)
    if not auth:
        return abortMessage("Invalid team name or password", c)

    # Update password
    new_hashed = bcrypt.hashpw(newPassword.encode('UTF_8'), bcrypt.gensalt())
    c.execute("UPDATE Team SET password = %s WHERE teamID = %s", (new_hashed, teamID))

    return success({}, c, db)


@puzzler_api.route("/changeMembers", methods=['POST'])
def changeMembers():
    releaseWaves()
    fail, content = parseJson(request, {"name": unicode, "password": unicode, "members": [{"name": unicode, "email": unicode}]})
    if fail:
        return content
    team_name = content["name"]
    password = content["password"].encode('UTF_8')
    members = content["members"]
    c = db.cursor()

    auth, teamID = authorized(team_name, password, c)
    if not auth:
        return abortMessage("Invalid team name or password", c)

    # No team members given
    if not members:
        return abortMessage("Team must have at least one member", c)

    # Too many team members
    if len(members) > TEAM_SIZE:
        return abortMessage("There are too many people on your team", c)

    # Delete current members
    c.execute("DELETE FROM Member WHERE teamID = %s", (teamID,))

    # Add each team member
    for member in members:
        member_name = member["name"]
        email = member["email"]
        # Bad email address
        if tooLong(member_name, "member_name"):
            return abortMessage(c, db, "Member name too long")
        elif not re.search(email_regex, email) or tooLong(email, "email"):
            return abortMessage(c, db, "Invalid email address")

        c.execute("INSERT INTO Member VALUES (%s, %s, %s)", (teamID, member_name, email))

    return success({}, c, db)


@puzzler_api.route("/viewOwnTeam", methods=['POST'])
def viewOwnTeam():
    releaseWaves()
    fail, content = parseJson(request, {"name": unicode, "password": unicode})
    if fail:
        return content
    team_name = content["name"]
    password = content["password"].encode('UTF_8')
    c = db.cursor()

    auth, teamID = authorized(team_name, password, c)
    if not auth:
        return abortMessage("Invalid team name or password", c)

    # Get guesses
    c.execute("SELECT guesses FROM Team WHERE teamID = %s", (teamID,))
    guesses, = c.fetchone()

    # Get all team members
    c.execute("SELECT name, email FROM Member WHERE teamID = %s", (teamID,))
    members = [{"name": rec[0], "email": rec[1]} for rec in c.fetchall()]

    return success({"guesses": guesses, "members": members, "name": team_name}, c)


@puzzler_api.route("/submitGuess", methods=['POST'])
def submitGuess():
    submit_time_dt = now() - datetime.timedelta(hours=4)
    submit_time = submit_time_dt.isoformat()
    releaseWaves()
    fail, content = parseJson(request, {"name": unicode, "password": unicode, "guess": unicode, "puzzle": unicode})
    if fail:
        return content
    team_name = content["name"]
    password = content["password"].encode('UTF_8')
    puzzle = content["puzzle"]
    guess = content["guess"]
    c = db.cursor()

    if tooLong(guess, "guess"):
        return abortMessage("Guess too long", c)

    # Bad credentials
    auth, teamID = authorized(team_name, password, c)
    if not auth:
        return abortMessage("Invalid team name or password", c)

    # Puzzle doesn't exist
    c.execute("SELECT answer, wave, currentPoints FROM Puzzle WHERE name = %s", (puzzle,))
    puzzle_rec = c.fetchone()
    if puzzle_rec == None:
        return abortMessage("Puzzle '%s' does not exist" % puzzle, c)
    answer, wave, currPoints = puzzle_rec

    # No solving unreleased or orphan puzzles
    c.execute("SELECT time FROM Wave WHERE name = %s", (wave,))
    wave_rec = c.fetchone()
    if wave_rec == None:
        return abortMessage("Puzzle '%s' does not exist" % puzzle, c)
    release_time = wave_rec[0].isoformat()
    if release_time > submit_time:
        return abortMessage("Puzzle '%s' does not exist" % puzzle, c)

    # Already solved
    c.execute("SELECT teamID, puzzle FROM Solve WHERE teamID = %s AND puzzle = %s", (teamID, puzzle))
    if c.fetchone() != None:
        return abortMessage("You have already solved this puzzle", c)

    # If you get here, your submission is valid
    # Out of guesses
    c.execute("SELECT guesses FROM Team WHERE teamID = %s", (teamID,))
    guesses, = c.fetchone()
    if guesses <= 0:
        return success({"isCorrect": "OutOfGuesses"}, c)

    # Normalize answer
        # Delete all whitespace, lowercase all alpha
        # It's the puzzler's fault if they feel like inputting other weird characters
    normal_guess = re.sub(r"\s+", "", guess.lower(), flags=re.UNICODE)

    c.execute("SELECT closed FROM Hunt")
    closed, = c.fetchone()

    # If you have guesses left
    # Decrement the number of guesses you have left
    # And save the guess
    c.execute("UPDATE Team SET guesses = %s WHERE teamID = %s", (guesses - 1, teamID))
    if not closed:
        c.execute("INSERT INTO Guess VALUES (%s, %s, %s, %s)", (teamID, puzzle, guess, submit_time))

    # Update stats table with another guess
    # Insert if not exists
        c.execute("UPDATE Stats SET guesses = guesses + 1 WHERE teamID = %s AND puzzle = %s RETURNING teamID, puzzle", (teamID, puzzle))
        if c.fetchone() == None:
            c.execute("INSERT INTO Stats VALUES (%s, %s, 0, null, 1)", (teamID, puzzle)) 

    # Incorrect answer
    if normal_guess != answer:
        # Don't touch solve table
        return success({"isCorrect": "Incorrect"}, c, db)

    if closed:
        return success({"isCorrect": "Correct"}, c, db)

    # Otherwise, correct answer
    # Calculate puzzle value by looking at hints released
        # Get set of hints associated to puzzle
        # Check in wave table via hint.wave to check whether hint has been released
        # Accumulate all penalties of all released hints, deduct from base value

    # Add entry to solve table
    c.execute("INSERT INTO Solve VALUES (%s, %s, %s)", (teamID, puzzle, submit_time))

    # Update stats table
    # Record to update should exist by this time
    # Calculate solve time
    release_time_dt = datetime.datetime.strptime(release_time, "%Y-%m-%dT%H:%M:%S")
    solve_time = (submit_time_dt - release_time_dt).total_seconds()
    c.execute("UPDATE Stats SET score = score + %s, solveTime = %s WHERE teamID = %s AND puzzle = %s", (currPoints, solve_time, teamID, puzzle))

    return success({"isCorrect": "Correct"}, c, db)


@puzzler_api.route("/viewHunt", methods=['POST'])
def viewHunt():
    releaseWaves()
    c = db.cursor()

    c.execute("SELECT name FROM Hunt")
    hunt_name, = c.fetchone()

    return success({"name": hunt_name}, c)


@puzzler_api.route("/viewPuzzles", methods=['POST'])
def viewPuzzles():
    releaseWaves()
    c = db.cursor()

    c.execute("SELECT name, number, currentPoints, wave, key FROM Puzzle WHERE released = true")
    puzzles = []
    for puzzle_rec in c.fetchall():
        puzzle_name, number, currPoints, wave, key = puzzle_rec
        # Get hints
        c.execute("SELECT number, key FROM Hint WHERE puzzle = %s AND released = true", (puzzle_name,))
        hints = [{"number": rec[0], "key": rec[1]} for rec in c.fetchall()]
        # Get wave release time
        c.execute("SELECT to_char(time, 'YYYY-MM-DDThh24:MI:SS') FROM Wave WHERE name = %s", (wave,))
        release_time, = c.fetchone()

        puzzles.append((release_time, number, {"name": puzzle_name, "number": number, "points": currPoints,
                        "wave": wave, "key": key, "hints": hints}))

    # Order puzzles
    ordered_puzzles = [tup[2] for tup in sorted(puzzles)]

    return success({"puzzles": ordered_puzzles}, c)


@puzzler_api.route("/viewTeamStats", methods=['POST'])
def viewTeamStats():
    releaseWaves()
    fail, content = parseJson(request, {"team": unicode})
    if fail:
        return content
    team_name = content["team"]
    c = db.cursor()

    # Get team name
    c.execute("SELECT teamID FROM Team WHERE name = %s", (team_name,))
    team_rec = c.fetchone()
    if team_rec == None:
        return abortMessage("Team '%s' does not exist" % team_name, c)
    teamID, = team_rec

    # Get data
    c.execute("SELECT puzzle, score, solveTime, guesses FROM Stats WHERE teamID = %s", (teamID,))
    puzzles = [{"puzzle": rec[0], "score": rec[1], "solveTime": rec[2], "guesses": rec[3]} for rec in c.fetchall()]

    return success({"puzzles": puzzles}, c)


@puzzler_api.route("/viewPuzzleStats", methods=['POST'])
def viewPuzzleStats():
    releaseWaves()
    fail, content = parseJson(request, {"puzzle": unicode})
    if fail:
        return content
    puzzle = content["puzzle"]
    c = db.cursor()

    # Check that puzzle exists
    c.execute("SELECT name FROM Puzzle WHERE name = %s", (puzzle,))
    if c.fetchone() == None:
        return abortMessage("Puzzle '%s' does not exist" % puzzle, c)

    # Get data
    c.execute("""SELECT Team.name, score, solveTime, Stats.guesses FROM Stats, Team
                WHERE Team.teamID = Stats.teamID AND puzzle = %s""", (puzzle,))
    teams = [{"team": rec[0], "score": rec[1], "solveTime": rec[2], "guesses": rec[3]} for rec in c.fetchall()]

    return success({"teams": teams}, c)


@puzzler_api.route("/viewTeamsStats", methods=['POST'])
def viewTeamsStats():
    releaseWaves()
    c = db.cursor()

    c.execute("""SELECT name, sum(score), count(solveTime), avg(solveTime)::int, sum(Stats.guesses)
                FROM stats, team WHERE Team.teamID = Stats.teamID group by name""")

    teams = [(-rec[1], rec[3], {"team": rec[0], "totalScore": rec[1], "totalSolves": rec[2],
                "avgSolveTime": rec[3], "guesses": rec[4] - rec[2]}) for i, rec in enumerate(c.fetchall())]
    ordered_teams = [tup[2] for tup in sorted(teams)]
    for i, team in enumerate(ordered_teams):
        team["rank"] = i+1

    # List teams that have not guessed yet
    n = len(teams)
    print n
    c.execute("SELECT name FROM Team WHERE teamID NOT IN (SELECT teamID FROM Stats)")
    ordered_teams += [{"rank": n+j+1, "team": rec[0], "totalScore": 0, "totalSolves": 0, "avgSolveTime": None, "guesses": 0}
                    for j, rec in enumerate(c.fetchall())]

    return success({"teams": ordered_teams}, c)


@puzzler_api.route("/viewPuzzlesStats", methods=['POST'])
def viewPuzzlesStats():
    releaseWaves()
    c = db.cursor()

    c.execute("""SELECT puzzle, count(solveTime), avg(solveTime)::int, sum(Stats.guesses) from Stats, Team
                WHERE Team.teamID = Stats.teamID group by puzzle""")

    puzzles = [{"puzzle": rec[0], "totalSolves": rec[1], "avgSolveTime": rec[2], "guesses": rec[3] - rec[1]} for rec in c.fetchall()]

    return success({"puzzles": puzzles}, c)


@puzzler_api.route("/viewMembers", methods=['POST'])
def viewMembers():
    releaseWaves()
    fail, content = parseJson(request, {"team": unicode})
    if fail:
        return content
    team_name = content["team"]
    c = db.cursor()

    c.execute("SELECT Member.name FROM Team, Member WHERE Member.teamID = Team.teamID AND Team.name")
    member_recs = c.fetchall()
    if not members:
        return abortMessage("Team '%s' does not exist" % team_name, c)

    members = sorted([{"member": rec[0]} for rec in member_recs])

    return success({"members": members}, c)
