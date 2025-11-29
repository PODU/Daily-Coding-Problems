# Day 671: Debounce. Each call cancels the pending timer and starts a fresh N-ms timer;
# f fires only after N ms of silence. Per-call O(1).
import threading
import time


def debounce(f, n_ms):
    state = {"timer": None}

    def wrapper(*args):
        if state["timer"]:
            state["timer"].cancel()
        state["timer"] = threading.Timer(n_ms / 1000.0, f, args=args)
        state["timer"].start()

    return wrapper


if __name__ == "__main__":
    g = debounce(lambda s: print("f called with:", s), 50)
    for s in ["a", "b", "c", "d", "e"]:
        g(s)  # rapid burst
    time.sleep(0.2)  # prints once: f called with: e
