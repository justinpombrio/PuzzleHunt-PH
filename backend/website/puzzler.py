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
    fail, content = parseJson(request, {"name": unicode})
    if fail:
        return content
    team_name = content["name"]
    c = db.cursor()

    # Get teamID, check exists
    c.execute("SELECT teamID FROM Team WHERE name = %s", (team_name,))
    teamID = c.fetchone()
    if teamID == None:
        return abortMessage("No team '%s'" % team_name, c)

    # Get all team members
    c.execute("SELECT name FROM Member WHERE teamID = %s", (teamID,))
    members = [{"name": name[0]} for name in c.fetchall()]

    return success({"members": members}, c)


@puzzler_api.route("/changePassword", methods=['POST'])
def changePassword():
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
    submit_time = now().isoformat(' ')
    fail, content = parseJson(request, {"name": unicode, "password": unicode, "puzzle": unicode, "guess": unicode})
    if fail:
        return content
    team_name = content["name"]
    password = content["password"].encode('UTF_8')
    puzzle_name = content["puzzle"]
    guess = content["guess"]
    c = db.cursor()

    if tooLong(guess, "guess"):
        return abortMessage("Guess too long", c)

    # Bad credentials
    auth, teamID = authorized(team_name, password, c)
    if not auth:
        return abortMessage("Invalid team name or password", c)

    # Puzzle doesn't exist
    c.execute("SELECT puzzleID, answer, waveID, points FROM Puzzle WHERE name = %s", (puzzle_name,))
    puzzle_rec = c.fetchone()
    if puzzle_rec == None:
        return abortMessage("Puzzle '%s' does not exist" % puzzle_name, c)
    puzzleID, answer, waveID, points = puzzle_rec

    # No solving unreleased or ghost puzzles
    c.execute("SELECT time FROM Wave WHERE waveID = %s", (waveID,))
    wave_rec = c.fetchone()
    if wave_rec == None:
        return abortMessage("Puzzle '%s' does not exist" % puzzle_name, c)
    release_time = wave_rec[0].isoformat(' ')
    if release_time > submit_time:
        return abortMessage("Puzzle '%s' does not exist" % puzzle_name, c)

    # Already solved
    c.execute("SELECT teamID, puzzleID FROM Solve WHERE teamID = %s AND puzzleID = %s", (teamID, puzzleID))
    if c.fetchone() != None:
        return abortMessage("You have already solved this puzzle", c)

    # If you get here, your submission is valid
    # Out of guesses
    c.execute("SELECT guesses FROM Team WHERE teamID = %s", (teamID,))
    guesses, = c.fetchone()
    if guesses <= 0:
        return success({"isCorrect": "outOfGuesses"}, c)

    # Normalize answer
        # Delete all whitespace, lowercase all alpha
        # It's the puzzler's fault if they feel like inputting other weird characters
    normal_guess = re.sub(r"\s+", "", guess.lower(), flags=re.UNICODE)

    # If you have guesses left
    # Decrement the number of guesses you have left
    # And save the guess
    c.execute("UPDATE Team SET guesses = %s WHERE teamID = %s", (guesses - 1, teamID))
    c.execute("INSERT INTO Guess VALUES (%s, %s, %s, %s)", (teamID, puzzleID, guess, submit_time))

    # Incorrect answer
    if normal_guess != answer:
        # Don't touch solve table
        return success({"isCorrect": "Incorrect"}, c, db)

    # Otherwise, correct answer
    # Calculate puzzle value by looking at hints released
        # Get set of hints associated to puzzleID
        # Check in wave table via hint.waveID to check whether hint has been released
        # Accumulate all penalties of all released hints, deduct from base value
    """
    c.execute("SELECT penalty, waveID FROM Hint WHERE puzzleID = %s", (puzzleID,))
    final_points = points
    for penalty, hint_waveID in c.fetchall():
        c.execute("SELECT time FROM wave WHERE waveID = %s", (hint_waveID,))
        time_rec = c.fetchone()
        if time_rec == None:
            # Ignore hints not associated to a wave
            continue
        if submit_time >= time_rec[0]:
            final_points -= penalty
    """

    # Add entry to solve table
    c.execute("INSERT INTO Solve VALUES (%s, %s, %s)", (teamID, puzzleID, submit_time))

    return success({"isCorrect": "Correct"}, c, db)




