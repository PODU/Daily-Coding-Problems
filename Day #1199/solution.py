# Day 1199: Job scheduler: call f after n ms using threading.Timer (sleeping worker thread).
# Time: O(1) to schedule; Space: O(1). Main joins the timer so the job runs first.
import threading

def schedule(f, n):
    t = threading.Timer(n / 1000.0, f)
    t.start()
    return t

if __name__ == "__main__":
    print("Scheduling job...")
    timer = schedule(lambda: print("Job executed after 100 ms"), 100)
    # Wait long enough for the scheduled job to fire before the program exits.
    timer.join()
