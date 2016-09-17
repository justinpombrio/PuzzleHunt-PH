import bcrypt
from flask import jsonify
import psycopg2

def tooLong(string, field):
    limits = {"team_name": 64,
        "member_name": 128,
        "email": 256,
        "wave_name": 64
    }
    return len(string) >= limits[field]

def abortMessage(message, cursor=None, conn=None):
    if cursor:
        cursor.close()
    if conn:
        conn.rollback()
    return jsonify({"status": "Failure", "message": message})

def success(data, cursor=None, conn=None):
    if cursor:
        cursor.close()
    if conn:
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
    elif type(json_data) != types:
        # Is not primitive
        return False
    return True
