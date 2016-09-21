import requests
import json
import time

PORT = 4000
DOMAIN = "http://localhost:%d" % PORT
JSON_HEADER = {"content-type": "application/json; charset=UTF-8"}

def make_request(route, payload, get_cookie=False, send_cookie=None):
    if send_cookie:
        r = requests.post(DOMAIN + route, data=json.dumps(payload), headers=JSON_HEADER, cookies={'session': send_cookie})
    else:
        r = requests.post(DOMAIN + route, data=json.dumps(payload), headers=JSON_HEADER)
    if get_cookie:
        cookie = r.cookies['session']
        return r.text, cookie
    return r.text

# Get hunt
print "Viewing hunt data"
payload = {}
print make_request("/viewPuzzles", payload)

# Register test team
print "Registering team"
payload = {"name": "MyTeam", "password": "ButThisIs", "members": [{"name": "Vikram", "email": "bla@gmail.com"}, {"name":"Siddharth", "email": "bla2@gmail.com"}]}
print make_request("/registerTeam", payload)

# View team
print "Viewing team"
payload = {"name": "MyTeam"}
print make_request("/viewTeam", payload)

# Change password
print "Changing password"
payload = {"name": "MyTeam", "password": "ButThisIs", "newPassword": "ButThisIs"}
print make_request("/changePassword", payload)

# Change members
print "Changing members"
payload = {"name": "MyTeam", "password": "ButThisIs", "members": [{"name": "Matt", "email": "mkibler@nd.edu"}, {"name": "Devin", "email": "dduffy@nd.edu"}]}
print make_request("/changeMembers", payload)

# Viewing own team
print "Viewing own team"
payload = {"name": "MyTeam", "password": "ButThisIs"}
print make_request("/viewOwnTeam", payload)

# Login as master
print "Master logging in"
payload = {"password": "password"}
resp, cookie = make_request("/login", payload, True)
print resp, cookie

# Get hunt
print "Getting hunt table"
payload = {}
print make_request("/getHunt", payload, False, cookie)

# Get all emails
print "Getting all emails"
payload = {}
print make_request("/getMembers", payload, False, cookie)

# Submit answer
print "Submitting garbage answer"
payload = {}
print make_request("/submitGuess", payload)

# Submit answer
print "Submitting answer"
payload = {"name": "MyTeam", "password": "ButThisIs", "puzzle": "", "guess": ""}
print make_request("/submitGuess", payload)

# Submit answer
print "Submitting answer"
payload = {"name": "MyTeam", "password": "ButTisIs", "puzzle": "", "guess": ""}
print make_request("/submitGuess", payload)

# Submit answer
print "Submitting answer"
payload = {"name": "MyTeam", "password": "ButThisIs", "puzzle": "", "guess": ""}
print make_request("/submitGuess", payload)

# Submit answer
print "Submitting answer"
payload = {"name": "MyTeam", "password": "ButThisIs", "puzzle": "GoodPuzzle", "guess": "godanswer"}
print make_request("/submitGuess", payload)

# Submit answer
print "Submitting answer"
payload = {"name": "MyTeam", "password": "ButThisIs", "puzzle": "BadPuzzle", "guess": "baddanswer"}
print make_request("/submitGuess", payload)

# Submit answer
print "Submitting answer"
payload = {"name": "MyTeam", "password": "ButThisIs", "puzzle": "GoodPuzzle", "guess": "goodanswer"}
print make_request("/submitGuess", payload)

# Get all puzzles
print "Getting puzzles"
payload = {}
print make_request("/getPuzzles", payload, False, cookie)

# Get all waves
print "Getting waves"
payload = {}
print make_request("/getWaves", payload, False, cookie)

# Setting one wave
print "Setting wave"
payload = {"waves": [{"name": "newwave", "time": "2010-3-3T3:03:03", "guesses":100}, {"name": "newwave2", "time": "2020-3-3T3:03:03", "guesses": 200}]}
print make_request("/setWaves", payload, False, cookie)
time.sleep(1)

# Set one puzzle
print "Setting one puzzle"
payload = {"puzzles": [{"name": "newpuzzle", "number": "1.4", "points": 20000, "wave": "bad wave", "answer": "", "key":""}]}
print make_request("/setPuzzles", payload, False, cookie)

# Getting puzzles
print "Getting puzzles"
payload = {}
print make_request("/getPuzzles", payload, False, cookie)

# Set one puzzle
print "Setting one puzzle"
payload = {"puzzles": [{"name": "newpuzzle", "number": "1.4", "points": 20000, "wave": "newwave", "answer": "yes", "key":""}]}
print make_request("/setPuzzles", payload, False, cookie)

# Get new puzzles
print "Getting Cool puzzles"
payload = {}
print make_request("/getPuzzles", payload, False, cookie)

# Get hints
print "Getting hints"
payload = {}
print make_request("/getHints", payload, False, cookie)

# Get waves
print "Getting waves"
payload = {}
print make_request("/getWaves", payload, False, cookie)

# Set hints
print "Setting hints"
payload = {"hints": [{"puzzle": "newpuzzle", "number": 1, "penalty": 2, "wave": "newwave", "key":""}]}
print make_request("/setHints", payload, False, cookie)

# Get hints
print "Getting hints"
payload = {}
print make_request("/getHints", payload, False, cookie)

# Get hunt
print "Getting hunt data"
payload = {}
print make_request("/getHunt", payload, False, cookie)

# Get hunt
print "Viewing hunt data"
payload = {}
print make_request("/viewHunt", payload)

# View puzzles
print "Viewing puzzles"
payload = {}
print make_request("/viewPuzzles", payload)

# Submit correct answer
#print "Submitting correct answer"
#payload = {"name": "MyTeam", "password": "ButThisIs", "puzzle": "newpuzzle", "guess": "yes"}
#print make_request("/submitGuess", payload)
