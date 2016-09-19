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

c.execute("INSERT INTO Wave VALUES (1, 'coolwave', 'May 21, 2010', 200, true)")
c.execute("INSERT INTO Puzzle VALUES (0, 'GoodPuzzle', '1.1', 20, 'goodanswer', 1, 'garbage')")
c.execute("INSERT INTO Puzzle VALUES (1, 'BaddPuzzle', '1.x', 200, 'badanswer', 100, 'garbage2')")
c.execute("INSERT INTO Hint VALUES (0, 1, 1, 1, 'bla')")
c.execute("INSERT INTO Hint VALUES (1, 1, 1, 1, 'bla')")

conn.commit()
c.close()
conn.close()

print "SET PUZZLES"
