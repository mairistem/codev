"""The Flask application: the routes the librarians use."""

from datetime import date

from flask import Flask, render_template_string

from library import loans

app = Flask(__name__)

OVERDUE_PAGE = """
<h1>Overdue loans</h1>
<table>
  <tr><th>Book</th><th>Member</th><th>Days late</th></tr>
  {% for loan in loans %}
  <tr><td>{{ loan.title }}</td><td>{{ loan.member }}</td><td>{{ loan.days_late }}</td></tr>
  {% endfor %}
</table>
"""


@app.get("/loans/overdue")
def overdue():
    return render_template_string(OVERDUE_PAGE, loans=loans.overdue(date.today()))
