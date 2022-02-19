from src.compiler.compiler import compile
from src.utils.error import throw
from src import config
from os.path import dirname, isfile
from termcolor import cprint, colored
import pyinotify

f = ""


def on_modify(event):
    try:
        compile(f, time_message=True)
    except Exception:
        print("Compilation Error.")


def on_delete(event):
    throw("File deleted, closing watcher.")
    exit(1)


def watch(filename: str):
    if not isfile(filename):
        filename = colored(filename, "yellow")
        throw(f'File not found: "{filename}"')
    if not filename.endswith(config.EXTENSION):
        filename = colored(filename, "yellow")
        throw(
            f'File must end with ".{config.EXTENSION}" extension: "{filename}"'
        )
    global f
    f = filename
    config.ERROR_NO_EXIT = True
    wm = pyinotify.WatchManager()
    wm.add_watch(dirname(filename), pyinotify.IN_MODIFY, on_modify)
    notifier = pyinotify.Notifier(wm)
    cprint(f"[Watching {filename}]", "blue")
    notifier.loop()