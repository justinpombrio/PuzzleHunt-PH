from flask import session, Blueprint, request
from website import db
from helpers import *
from datetime import datetime as dt

master_api = Blueprint('master_api', __name__)

@master_api.route("/logout", methods=['POST'])
def logout():
    session.pop('username', None)
    return success({})


@master_api.route("/login", methods=['POST'])
def login():
    fail, content = parseJson(request, {"password": unicode})
    if fail:
        return content
    password = content["password"].encode('UTF_8')
    c = db.cursor()

    c.execute("SELECT password FROM Hunt");
    stored_hash = c.fetchone()[0]
    hashed = bcrypt.hashpw(password, stored_hash)
    if hashed != stored_hash:
        return abortMessage("Incorrect password", c)

    session['username'] = "master"
    return success({}, c)    


@master_api.route("/getHunt", methods=['POST'])
def getHunt():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    c = db.cursor()

    c.execute("SELECT name, teamSize, initGuesses FROM Hunt")
    hunt_name, team_size, init_guesses = c.fetchone()
    return success({"name": hunt_name, "teamSize": team_size, "initGuesses": init_guesses}, c)


@master_api.route("/setHunt", methods=['POST'])
def setHunt():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    fail, content = parseJson(request, {"name": unicode, "teamSize": unicode, "initGuesses": unicode})
    if fail:
        return content
    hunt_name = content["name"]
    team_size = content["teamSize"]
    init_guesses = content["initGuesses"]
    c = db.cursor()

    # TODO: Limits on the above values?

    c.execute("UPDATE Hunt SET name = %s, teamSize = %s, initGuesses = %s", (hunt_name, team_size, init_guesses))

    return success({}, c, db)


@master_api.route("/getWaves", methods=['POST'])
def getWaves():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    c = db.cursor()

    c.execute("SELECT name, time, guesses, visible FROM Wave")
    waves = [{"name": rec[0], "time": rec[1], "guesses": rec[2],
            "visible": rec[3]} for rec in c.fetchall()]
    return success({"waves": waves}, c)


@master_api.route("/setWaves", methods=['POST'])
def setWaves():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    fail, content = parseJson(request, {"waves": [{"name": unicode, "time": dt, "guesses": int, "visible": bool}]})
    if fail:
        return content
    waves = content["waves"]
    c = db.cursor()

    # Check uniqueness of wave names
    wave_names = map(lambda w: w["name"], waves)
    if len(wave_names) != len(set(wave_names)):
        return abortMessage("Wave names must be unique", c)

    # Delete existing waves
    c.execute("DELETE FROM Wave")

    # Insert new waves
    for n, wave in enumerate(waves):
        wave_name = wave["name"]
        release_time = wave["time"]
        guesses = wave["guesses"]
        visible = wave["visible"]
        if tooLong(wave_name, "wave_name"):
            return abortMessage("Wave name too long", c, db)
        c.execute("INSERT INTO Wave VALUES (%s, %s, %s, %s, %s)", (n, wave_name, release_time, guesses, visible))

    return success({}, c, db)


@master_api.route("/getPuzzles", methods=['POST'])
def getPuzzles():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    c = db.cursor()

    c.execute("SELECT name, number, points, answer, key, waveID FROM Puzzle")
    puzzles = []
    # If a puzzle is no longer associated with a wave, null out its wave name
    for rec in c.fetchall():
        puzzle_name, number, points, answer, key, waveID = rec
        c.execute("SELECT name FROM Wave WHERE waveID = %s", (waveID,))
        wave_rec = c.fetchone()
        if wave_rec == None:
            wave_name = None
        else:
            wave_name, = wave_rec
        puzzles.append({"name": puzzle_name, "number": number, "points": points,
                "wave": wave_name, "answer": answer, "key": key})

    return success({"puzzles": puzzles}, c)


@master_api.route("/setPuzzles", methods=['POST'])
def setPuzzles():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    fail, content = parseJson(request, {"puzzles": [{"name": unicode, "number": unicode, "points": int,
                                    "wave": unicode, "answer": unicode, "key": unicode}]})
    if fail:
        return content
    puzzles = content["puzzles"]
    c = db.cursor()

    # Check uniqueness of wave names
    puzzle_names = map(lambda p: p["name"], puzzles)
    if len(puzzle_names) != len(set(puzzle_names)):
        return abortMessage("Wave names must be unique", c)

    # Delete existing waves
    c.execute("DELETE FROM Puzzle")

    # Abort if a wave doesn't exist
    for n, puzzle in enumerate(puzzles):
        puzzle_name = puzzle["name"]
        number = puzzle["number"]
        points = puzzle["points"]
        wave_name = puzzle["wave"]
        answer = puzzle["answer"]
        key = puzzle["key"]
        c.execute("SELECT waveID FROM Wave WHERE name = %s", (wave_name,))
        wave_rec = c.fetchone()
        if wave_rec == None:
            return abortMessage("Wave '%s' does not exist" % wave_name, c, db)
        waveID, = wave_rec
        if tooLong(puzzle_name, "puzzle_name"):
            return abortMessage("Puzzle name too long", c, db)
        if tooLong(number, "number"):
            return abortMessage("Puzzle number too long", c, db)
        c.execute("INSERT INTO Puzzle VALUES (%s, %s, %s, %s, %s, %s, %s)", (n, puzzle_name, number, points, answer, waveID, key))

    return success({}, c, db)


@master_api.route("/getHints", methods=['POST'])
def getHints():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    c = db.cursor()

    c.execute("SELECT puzzleID, number, penalty, waveID, key FROM Hint")
    hints = []
    # If a puzzle is no longer associated with a wave, null out its wave name
    for rec in c.fetchall():
        puzzleID, number, penalty, waveID, key = rec
        print waveID
        c.execute("SELECT name FROM Wave WHERE waveID = %s", (waveID,))
        wave_rec = c.fetchone()
        if wave_rec == None:
            wave_name = None
            print "No wave name!"
        else:
            wave_name, = wave_rec
        c.execute("SELECT name FROM Puzzle WHERE puzzleID = %s", (puzzleID,))
        puzzle_rec = c.fetchone()
        if puzzle_rec == None:
            puzzle_name = None
        else:
            puzzle_name, = puzzle_rec
        hints.append({"puzzle": puzzle_name, "number": number, "penalty": penalty,
                "wave": wave_name, "key": key})

    return success({"hints": hints}, c)


@master_api.route("/setHints", methods=['POST'])
def setHints():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    fail, content = parseJson(request, {"hints": [{"puzzle": unicode, "number": int, "penalty": int,
                                    "wave": unicode, "key": unicode}]})
    if fail:
        return content
    hints = content["hints"]
    c = db.cursor()

    # Delete existing waves
    c.execute("DELETE FROM Hint")

    # Abort if a wave or puzzle doesn't exist
    for hint in hints:
        puzzle_name = hint["puzzle"]
        number = hint["number"]
        penalty = hint["penalty"]
        wave_name = hint["wave"]
        key = hint["key"]
        c.execute("SELECT waveID FROM Wave WHERE name = %s", (wave_name,))
        wave_rec = c.fetchone()
        if wave_rec == None:
            return abortMessage("Wave '%s' does not exist" % wave_name, c, db)
        waveID, = wave_rec
        print waveID
        c.execute("SELECT puzzleID FROM Puzzle WHERE name = %s", (puzzle_name,))
        puzzle_rec = c.fetchone()
        if puzzle_rec == None:
            return abortMessage("Puzzle '%s' does not exist" % puzzle_name, c, db)
        puzzleID, = puzzle_rec
        c.execute("INSERT INTO Hint VALUES (%s, %s, %s, %s, %s)", (puzzleID, number, penalty, waveID, key))

    return success({}, c, db)


@master_api.route("/getMembers", methods=['POST'])
def getMembers():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    c = db.cursor()

    c.execute("SELECT name, email FROM Member")
    name_emails = [{"name": rec[0], "email": rec[1]} for rec in c.fetchall()]

    return success({"members": name_emails}, c)
