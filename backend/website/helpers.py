import bcrypt
from flask import jsonify
import psycopg2
import datetime

SIZE_LIMITS = {"team_name": 64,
        "member_name": 128,
        "email": 256,
        "wave_name": 64,
        "guess": 64,
        "puzzle_name": 64,
        "number": 64
    }

def tooLong(string, field):
    return len(string) >= SIZE_LIMITS[field]

def abortMessage(message, cursor=None, conn=None):
    print "Failure"
    if cursor:
        cursor.close()
    if conn:
        conn.rollback()
    resp = jsonify({"status": "Failure", "message": message})
    resp.headers['Access-Control-Allow-Origin'] = "*"
    return resp

def success(data, cursor=None, conn=None):
    print "Success"
    if cursor:
        cursor.close()
    if conn:
        conn.commit()
    data["status"] = "Success"
    resp = jsonify(data)
    resp.headers['Access-Control-Allow-Origin'] = "*"
    return resp

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

# Take dictionary of data and dictionary of types
def typeCheck(json_data, types):
    if type(json_data) == dict == type(types):
        # Is dictionary
        # Check that set of fields is the same
        if json_data.keys() != types.keys():
            return False
        # Recursive typecheck
        for field in json_data:
            if not typeCheck(json_data[field], types[field]):
                return False
    elif type(json_data) == list == type(types):
        # Is list
        # Must be of length 1
        if len(types) != 1:
            return False
        for elem in json_data:
            if not typeCheck(elem, types[0]):
                return False
    elif types == datetime.datetime:
        print "Trying datetime"
        # Try parsing datetime
        if type(json_data) != unicode:
            return False
        try:
            datetime.datetime.strptime(json_data, "%Y-%m-%d %H:%M:%S")
        except ValueError:
            print "Did not parse"
            return False
    elif type(json_data) != types:
        print type(json_data)
        print types
        # Is not primitive
        return False
    return True

# Validate and type check JSON
# Return failure response if fails
# Otherwise return JSON content
def parseJson(rqst, type_sig):
    content = rqst.get_json(silent=True)
    if content == None:
        return True, abortMessage("Internal error: Invalid JSON")
    if not typeCheck(content, type_sig):
        return True, abortMessage("Internal error: Type check failed")
    return False, content
