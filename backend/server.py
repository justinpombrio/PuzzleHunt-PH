from flask import Flask
from flask_cors import CORS
import psycopg2
import bcrypt

from website import db
from website.master import master_api
from website.puzzler import puzzler_api

app = Flask(__name__)
CORS(app)

app.register_blueprint(master_api)
app.register_blueprint(puzzler_api)

if __name__ == '__main__':
    # Get secret key
    c = db.cursor()
    c.execute("SELECT secretKey FROM Hunt")
    app.secret_key, = c.fetchone()

    # Start application server
    c.close()
    app.run(host='0.0.0.0', port=4000)
    db.close()













