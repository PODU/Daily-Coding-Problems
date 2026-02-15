# Day 1073: Job scheduler: schedule f after n ms using threading.Timer. O(1) schedule, joins timer thread for clean exit.
import threading

def schedule(f, delay_ms):
    """Schedule f to run after delay_ms milliseconds."""
    t = threading.Timer(delay_ms / 1000.0, f)
    t.start()
    return t

print("Scheduling job...")
done = threading.Event()

def job():
    print("Job executed!")
    done.set()

t = schedule(job, 100)
done.wait()
t.join()
