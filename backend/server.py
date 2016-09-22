from flask import Flask
import psycopg2
import bcrypt

from website import db
from website.master import master_api
from website.puzzler import puzzler_api
from website.helpers import nocache

app = Flask(__name__, static_folder="/home/ec2-user/PuzzleHunt-PH/web/")

app.register_blueprint(master_api)
app.register_blueprint(puzzler_api)

@app.route("/", methods=['GET'])
@nocache
def index():
    return app.send_static_file("index.xml")

@app.route("/<path:path>", methods=['GET'])
@nocache
def web(path):
    return app.send_static_file(path)

if __name__ == '__main__':
    # Get secret key
    c = db.cursor()
    c.execute("SELECT secretKey FROM Hunt")
    app.secret_key, = c.fetchone()

    # Start application server
    c.close()
    app.run(host='0.0.0.0', port=80)
    db.close()













