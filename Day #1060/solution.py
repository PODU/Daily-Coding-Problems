# Day 1060: Debounce: wrap f so it runs only after N ms of silence; each call resets the timer.
# Uses threading.Timer. Time: O(1) per call, Space: O(1).
import threading


def debounce(f, n_ms):
    delay = n_ms / 1000.0
    lock = threading.Lock()
    state = {"timer": None}

    def wrapped(*args, **kwargs):
        with lock:
            if state["timer"] is not None:
                state["timer"].cancel()
            state["timer"] = threading.Timer(delay, f, args, kwargs)
            state["timer"].start()

    return wrapped


if __name__ == "__main__":
    import time

    calls = {"n": 0}

    def target(x):
        calls["n"] += 1
        print(f"f called with {x}; total calls = {calls['n']}")

    debounced = debounce(target, 100)
    for i in range(1, 6):  # burst of 5 calls
        debounced(i)

    time.sleep(0.3)  # wait > N ms
    print(f"done; f ran {calls['n']} time(s)")
