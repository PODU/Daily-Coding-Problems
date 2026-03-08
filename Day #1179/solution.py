# Day 1179: Debounce - call f only after N ms elapse with no further invocations.
# Each call cancels the pending threading.Timer and starts a fresh one.
# Time O(1) per call.
import threading
import time


def debounce(f, n_ms):
    delay = n_ms / 1000.0
    state = {"timer": None}
    lock = threading.Lock()

    def wrapper(*args, **kwargs):
        with lock:
            if state["timer"] is not None:
                state["timer"].cancel()
            state["timer"] = threading.Timer(delay, lambda: f(*args, **kwargs))
            state["timer"].start()

    return wrapper


if __name__ == "__main__":
    count = [0]

    def f():
        count[0] += 1

    debounced = debounce(f, 100)
    for _ in range(5):          # rapid burst, every 20ms
        debounced()
        time.sleep(0.02)
    time.sleep(0.3)
    print(f"f executed {count[0]} time(s)")  # 1
