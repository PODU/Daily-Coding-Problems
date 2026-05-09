# Day 1499: Job scheduler that calls f after n ms using threading.Timer.
# Time O(1) to schedule, Space O(1).
import threading


def schedule(f, n_ms):
    timer = threading.Timer(n_ms / 1000.0, f)
    timer.start()
    return timer


if __name__ == "__main__":
    timer = schedule(lambda: print("Job executed after delay"), 100)
    timer.join()  # wait for the scheduled job before exiting
