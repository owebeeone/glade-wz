# decisions

Your answers to the Glade decision graph live here, one file per notebook.

## What is in this folder

The decision graph is a list of questions about Glade, each with the answers on
offer. The questions live in the Gyld checkout
(`gyld/examples/glade-decisions.gyld.py`). Nothing in this folder changes them.

A notebook is a small Gyld file that says "start from those questions and add
these answers". When you fork or link a notebook in the decision desk, or answer
a question in one, the desk writes the notebook here:

    glade-decisions-<notebook>.gyld.py

Each answer is a few lines — a class that names the question it decides and the
alternative it picks:

    class VersionPinRuling(Ruling):
        """<when, who, and why, in your words>"""
        principal = "gianni"
        stamp = "2026-09-13T01:00:00Z"
        decides = Decides[VersionPin]
        selects = Selects[BumpToCurrent]

and one line further down, in the notebook's own class, that puts the answer in
the notebook:

        version_pin_ruling = use(VersionPinRuling)

One notebook can hold as many answers as you like: each time you answer a
question, the desk adds those few lines to the notebook and leaves everything
else in the file exactly as it was, comments and all. Answering the same question
twice in one notebook is refused, and it says so plainly — to change an answer,
open the file, edit the lines above and press Rebuild.

Gyld checks every notebook each time the desk rebuilds. It refuses an answer
that is not one of the offered ones, a second answer to the same question, and
an answer to a question whose prerequisites are still open.

When that happens the desk puts the notebook back exactly as it was, so nothing
here is left half-answered and every other stream goes on working. The Decide
window tells you why, in Gyld's own words, and the answer you typed stays in the
box so you can fix it and send it again.

If you answer inside one of the sample notebooks that ship with Gyld
(`stream-a`, `stream-b`, `fork-a`), the desk puts your copy here and leaves the
shipped sample untouched. Your copy is the one the desk uses from then on.

## Git is the history

The desk writes files and never runs git. A new answer shows up in `git status`
as a changed file. Commit it when you mean it: `git log` is then the history of
your rulings, and `git checkout` takes one back. Deleting a notebook's file
deletes the notebook at the next rebuild.

## How the desk finds this folder

`gyld-ui.py` passes this folder to the supplier as `--decisions-root`
(default: `<glade-wz>/decisions`). Build output and scratch files stay under
`~/.gyld-ui/`; only notebooks are written here.
