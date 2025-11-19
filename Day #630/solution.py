# Day 630: Job scheduler: run f after n ms using threading.Timer (background thread).
# Schedule is O(1); main joins the timer thread to wait for the job before exiting.
import threading


def schedule(f, n_ms):
    timer = threading.Timer(n_ms / 1000.0, f)
    timer.start()
    return timer


if __name__ == "__main__":
    t = schedule(lambda: print("Job executed after 100 ms"), 100)
    t.join()  # wait for the job to run before exiting
    print("Main: job completed, exiting")
