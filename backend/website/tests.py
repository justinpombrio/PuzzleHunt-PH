import requests
import json

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
