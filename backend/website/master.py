from flask import session, Blueprint
from website import db
from helpers import *

master_api = Blueprint('master_api', __name__)

@master_api.route("/logout", methods=['POST'])
def logout():
    session.pop('username', None)
    return success({})


@master_api.route("/login", methods=['POST'])
def login():
    content = request.get_json()
    if content == None:
        return abortMessage("Internal error: Invalid JSON")
    if not typeCheck(content, {"password": unicode}):
        return abortMessage("Internal error: Type check failed")
    password = content["password"].encode('UTF_8')
    c = db.cursor()

    c.execute("SELECT password FROM Hunt");
    stored_hash = c.fetchone()[0]
    hashed = bcrypt.hashpw(password, stored_hash)
    if hashed != stored_hash:
        return abortMessage("Incorrect password", c)

    session['username'] = "master"
    return success({}, c)    


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

    content = request.get_json()
    if content == None:
        return abortMessage("Internal error: Invalid JSON")
    if not typeCheck(content, {"name": unicode, "password": unicode, "members": [{"name": unicode, "email": unicode}]}):
        return abortMessage("Internal error: Type check failed")
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
        if tooLong(wave_name, "wave_name"):
            return abortMessage("Wave name too long", c, db)
        c.execute("INSERT INTO Wave VALUES (%s, %s, %s, %s)", (n, wave_name, release_time, guesses))

    return success({}, c, db)


@master_api.route("/getPuzzles", methods=['POST'])
def getPuzzles():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    c = db.cursor()

    c.execute("""SELECT Puzzle.name, Puzzle.number, Puzzle.points,
            Wave.name, Puzzle.answer, Puzzle.key
            FROM Puzzle, Wave WHERE Puzzle.waveID = Wave.waveID""")
    puzzles = [{"name": rec[0], "number": rec[1], "points": rec[2],
            "wave": rec[3], "answer": rec[4], "key": rec[5]} for rec in c.fetchall()]

    return success({"puzzles": puzzles}, c)


@master_api.route("/setPuzzles", methods=['POST'])
def setPuzzles():
    if 'username' not in session:
        return abortMessage("Unauthorized")

    content = request.get_json()
    puzzles = content["puzzles"]
    c = db.cursor()
