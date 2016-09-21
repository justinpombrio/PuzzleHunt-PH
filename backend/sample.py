import psycopg2
import binascii
from sys import exit

try:
    conn = psycopg2.connect("dbname='crums' user='postgres' host='localhost' password='password'")
except:
    print "Can't connect to database!"
    exit(1)

c = conn.cursor()

c.execute("DELETE FROM Wave")
c.execute("DELETE FROM Puzzle")
c.execute("DELETE FROM Solve")
c.execute("DELETE FROM Hint")

c.execute("INSERT INTO Wave VALUES ('coolwave', 'May 21, 2010', 200, true)")
c.execute("INSERT INTO Puzzle VALUES ('GoodPuzzle', '1.1', 20, 20, 'goodanswer', 'coolwave', 'garbage', true)")
c.execute("INSERT INTO Puzzle VALUES ('BaddPuzzle', '1.x', 200, 200, 'badanswer', 'crap', 'garbage2', true)")
c.execute("INSERT INTO Hint VALUES ('GoodPuzzle', 1, 1, 'coolwave', 'bla', true)")
c.execute("INSERT INTO Hint VALUES ('BaddPuzzle', 1, 1, 'coolwave', 'bla', true)")

conn.commit()
c.close()
conn.close()

print "SET PUZZLES"
