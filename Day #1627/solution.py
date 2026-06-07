# Day 1627: Debounce f by N ms. Each call resets an N-ms timer; f fires
# only after a quiet gap of N ms. Time O(1) per call.
import threading
import time


def debounce(f, n_ms):
    state = {"timer": None}

    def wrapper(*args, **kwargs):
        if state["timer"] is not None:
            state["timer"].cancel()
        state["timer"] = threading.Timer(n_ms / 1000.0, f, args, kwargs)
        state["timer"].start()

    return wrapper


if __name__ == "__main__":
    calls = [0]
    g = debounce(lambda: calls.__setitem__(0, calls[0] + 1), 100)
    g(); g(); g()
    time.sleep(0.2)
    print(f"f was called {calls[0]} time(s)")
