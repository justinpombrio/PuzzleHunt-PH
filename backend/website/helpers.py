import bcrypt
from flask import jsonify, make_response
import psycopg2
import datetime
from website import db, START_TIME
from functools import wraps, update_wrapper

SIZE_LIMITS = {"team_name": 64,
        "member_name": 128,
        "email": 256,
        "wave_name": 64,
        "guess": 64,
        "puzzle_name": 64,
        "number": 64
    }

NEXT_CHECK = START_TIME
FREQ_CHECK = datetime.timedelta(0, 1)

def tooLong(string, field):
    return len(string) >= SIZE_LIMITS[field]

def releaseWaves():
    global NEXT_CHECK
    curr_time = datetime.datetime.now() - datetime.timedelta(hours=4)
    if curr_time < NEXT_CHECK:
        return

    # Select waves that have not been released but should be
    c = db.cursor()

    c.execute("UPDATE Wave SET released = true WHERE time <= %s AND released = false RETURNING time, name, guesses, released", (curr_time,))
    print c.mogrify("UPDATE Wave SET released = true WHERE time <= %s AND released = false RETURNING time, name, guesses, released", (curr_time,))
    wave_recs = c.fetchall()

    # Release all waves in order
    for wave_rec in sorted(wave_recs):
        releaseWave(wave_rec, c)

    # Update next check time
    NEXT_CHECK = datetime.datetime.now() - datetime.timedelta(hours=4) + FREQ_CHECK

    db.commit()
    c.close()

def releaseWave(wave_rec, c):
    _, wave, guesses, released = wave_rec

    # Update guesses
    c.execute("UPDATE Team SET guesses = %s", (guesses,))

    # Release hints
    c.execute("UPDATE Hint SET released = true WHERE wave = %s RETURNING puzzle, penalty", (wave,))
    penalties = c.fetchall()
    # Deduct points
    for puzzle, penalty in penalties:
        c.execute("UPDATE Puzzle SET currentPoints = currentPoints - %s WHERE name = %s", (penalty, puzzle))

    # Release puzzles
    c.execute("UPDATE Puzzle SET released = true WHERE wave = %s RETURNING name, wave, released", (wave,))

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
    elif type(json_data) == int == types:
        if json_data < 0:
            print "Negative value!"
            return False
    elif types == datetime.datetime:
        print "Trying datetime"
        # Try parsing datetime
        if type(json_data) != unicode:
            return False
        try:
            datetime.datetime.strptime(json_data, "%Y-%m-%dT%H:%M:%S")
        except:
            try:
                datetime.datetime.strptime(json_data, "%Y-%m-%dT%H:%M")
            except:
                print "Did not parse"
                return False
            return True
    elif type(json_data) != types:
        # Is not primitive
        print json_data
        print type(json_data)
        print types
        return False
    return True

# Validate and type check JSON
# Return failure response if fails
# Otherwise return JSON content
def parseJson(rqst, type_sig):
    content = rqst.get_json(silent=True)
    print content
    if content == None:
        return True, abortMessage("Internal error: Invalid JSON")
    if not typeCheck(content, type_sig):
        return True, abortMessage("Internal error: Type check failed")
    return False, content

# Code copied from http://arusahni.net/blog/2014/03/flask-nocache.html
# No caching
def nocache(view):
    @wraps(view)
    def no_cache(*args, **kwargs):
        response = make_response(view(*args, **kwargs))
        response.headers['Last-Modified'] = datetime.datetime.now()
        response.headers['Cache-Control'] = 'no-store, no-cache, must-revalidate, post-check=0, pre-check=0, max-age=0'
        response.headers['Pragma'] = 'no-cache'
        response.headers['Expires'] = '-1'
        return response
        
    return update_wrapper(no_cache, view)
