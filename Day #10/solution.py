# Day 10: Job scheduler: use threading.Timer to invoke f after n milliseconds.
# schedule: O(1); each job runs on its own timer thread.
import threading


def schedule(f, n):
    timer = threading.Timer(n / 1000.0, f)
    timer.start()
    return timer


if __name__ == "__main__":
    done = threading.Event()

    def job():
        print("Executed after delay!")
        done.set()

    schedule(job, 100)
    done.wait(timeout=1.0)  # let the job fire before exit
