from flask import Flask, jsonify, request
import psycopg2
import bcrypt
import httplib
import re

app = Flask(__name__)
db = psycopg2.connect("dbname='crums' user='postgres' host='localhost' password='password'")
email_regex = r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)"

# Limits
def tooLong(string, field):
    limits = {"team_name": 64,
        "member_name": 128,
        "email": 256
    }
    try: 
        return len(string) >= limits[field]
    except KeyError:
        print "Internal error: invalid field"

def abortMessage(cursor, conn, message):
    if cursor:
        cursor.close()
    if conn:
        conn.rollback()
    return jsonify({"status": "Failure", "message": message})

def success(cursor, conn, data):
    cursor.close()
    conn.commit()
    data["status"] = "Success"
    return jsonify(data)

def authorized(team_name, password, cursor):
    # Get teamID, check exists
    cursor.execute("SELECT teamID, password FROM Team WHERE name = %s", (team_name,))
    rec = cursor.fetchone()
    if rec == None:
        return (False, None)
    teamID = rec[0]
    stored_hash = rec[1]

    # Get password
    hashed = bcrypt.hashpw(password, stored_hash)
    if hashed != stored_hash:
        return (False, None)

    return (True, teamID)
    

@app.route("/")
def test():
    return "Hello world!"

@app.route("/registerTeam", methods=['POST'])
def registerTeam():
    # Retrieve data
    content = request.get_json()
    team_name = content["name"]
    password = content["password"].encode('UTF_8')
    members = content["members"]
    c = db.cursor()

    # Team name too long
    if tooLong(team_name, "team_name"):
        return abortMessage(c, db, "Team name too long")

    # No team members given
    if not members:
        return abortMessage(c, db, "Team must have at least one member")

    # If team exists, abort
    c.execute("SELECT teamID FROM Team WHERE name = %s", (team_name,));
    if c.fetchone():
        return abortMessage(c, db, "Team '%s' already exists" % team_name)

    # Add new team, get teamID to add members
    hashed = bcrypt.hashpw(password, bcrypt.gensalt())
    c.execute("INSERT INTO Team VALUES (DEFAULT, %s, %s, %s) RETURNING teamID", (hashed, team_name, 0))
    teamID = c.fetchone()[0]

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

    return success(c, db, {})


@app.route("/viewTeam", methods=['POST'])
def viewTeam():
    content = request.get_json()
    team_name = content["name"]
    c = db.cursor()

    # Get teamID, check exists
    c.execute("SELECT teamID FROM Team WHERE name = %s", (team_name,))
    teamID = c.fetchone()
    if teamID == None:
        return abortMessage(c, db, "No team '%s'" % team_name)

    # Get all team members
    c.execute("SELECT name FROM Member WHERE teamID = %s", (teamID,))
    members = [{"name": name[0]} for name in c.fetchall()]

    return success(c, db, {"members": members})


@app.route("/changePassword", methods=['POST'])
def changePassword():
    content = request.get_json()
    team_name = content["name"]
    password = content["password"].encode('UTF_8')
    newPassword = content["newPassword"]
    c = db.cursor()

    auth, teamID = authorized(team_name, password, c)
    if not auth:
        return abortMessage(c, db, "Invalid team name or password")

    # Update password
    new_hashed = bcrypt.hashpw(newPassword.encode('UTF_8'), bcrypt.gensalt())
    c.execute("UPDATE Team SET password = %s WHERE teamID = %s", (new_hashed, teamID))
    
    return success(c, db, {})


@app.route("/changeMembers", methods=['POST'])
def changeMembers():
    content = request.get_json()
    team_name = content["name"]
    password = content["password"].encode('UTF_8')
    members = content["members"]
    c = db.cursor()

    auth, teamID = authorized(team_name, password, c)
    if not auth:
        return abortMessage(c, db, "Invalid team name or password")

    # No team members given
    if not members:
        return abortMessage(c, db, "Team must have at least one member")

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

    return success(c, db, {})


@app.route("/viewOwnTeam", methods=['POST'])
def viewOwnTeam():
    content = request.get_json()
    team_name = content["name"]
    password = content["password"].encode('UTF_8')
    c = db.cursor()

    auth, teamID = authorized(team_name, password, c)
    if not auth:
        return abortMessage(c, db, "Invalid team name or password")

    # Get guesses
    c.execute("SELECT guesses FROM Team WHERE teamID = %s", (teamID,))
    guesses = c.fetchone()[0]

    # Get all team members
    c.execute("SELECT name, email FROM Member WHERE teamID = %s", (teamID,))
    members = [{"name": rec[0], "email": rec[1]} for rec in c.fetchall()]

    return success(c, db, {"guesses": guesses, "members": members})

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=4000)
    db.close()













