from flask import Flask, render_template_string

from recipes import store

app = Flask(__name__)

HOME = """<h1>Cookbook</h1><ul>{% for r in recipes %}<li><a href="/r/{{ r.id }}">{{ r.title }}</a></li>{% endfor %}</ul>"""


@app.get("/")
def home():
    return render_template_string(HOME, recipes=store.by_title(store.connect()))
