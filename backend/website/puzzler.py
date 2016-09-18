from flask import Blueprint, request
from helpers import *
from website import db
import re
import bcrypt

puzzler_api = Blueprint('puzzler_api', __name__)

c = db.cursor()
c.execute("SELECT teamSize, initGuesses FROM Hunt")
TEAM_SIZE, INIT_GUESSES = c.fetchone()
email_regex = r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)"

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


#@puzzler_api.route("/viewPuzzles")
