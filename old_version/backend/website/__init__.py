import psycopg2
import datetime

db = psycopg2.connect("dbname='crums' user='postgres' host='localhost' password='password'")
START_TIME = datetime.datetime.utcnow() - datetime.timedelta(hours=4)
