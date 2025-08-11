# Day 105: Debounce. Each call cancels the previously scheduled timer and starts a
# fresh N-ms timer, so f only fires after N ms of quiet. O(1) per call.
import threading
import time


def debounce(f, n_ms):
    state = {'timer': None}

    def wrapper(*args, **kwargs):
        if state['timer'] is not None:
            state['timer'].cancel()
        state['timer'] = threading.Timer(n_ms / 1000.0, f, args, kwargs)
        state['timer'].start()

    return wrapper


if __name__ == "__main__":
    calls = {'n': 0}

    def f():
        calls['n'] += 1
        print("f called")

    d = debounce(f, 100)
    d(); d(); d()              # 3 rapid calls within the window
    time.sleep(0.3)            # wait past the debounce interval
    print("Total calls to f:", calls['n'])  # 1
