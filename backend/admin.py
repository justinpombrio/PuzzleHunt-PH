import psycopg2
import bcrypt
import os
import binascii
from sys import exit

try:
    conn = psycopg2.connect("dbname='crums' user='postgres' host='localhost' password='password'")
except:
    print "Can't connect to database!"
    exit(1)

NAME = "CRUMS 2016"
TEAM_SIZE = 4
INIT_GUESSES = 100
PASSWORD = raw_input("Enter master password: ").encode('UTF_8')
SECRET_KEY = binascii.b2a_hex(os.urandom(24))
CLOSED = False

hashed = bcrypt.hashpw(PASSWORD, bcrypt.gensalt())

record = (NAME, TEAM_SIZE, INIT_GUESSES, hashed, SECRET_KEY, CLOSED)

c = conn.cursor()
c.execute("DELETE FROM Hunt");
c.execute("INSERT INTO Hunt VALUES (%s, %s, %s, %s, %s, %s)", record)
conn.commit()
c.close()
conn.close()

print "SET PASSWORD"
