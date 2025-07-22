// Job scheduler: a single timer thread runs a delay queue ordered by due time.
// schedule: O(log k); jobs fire at their scheduled instant.
import java.util.concurrent.DelayQueue;
import java.util.concurrent.Delayed;
import java.util.concurrent.TimeUnit;

public class Solution {
    static class Job implements Delayed {
        final Runnable f;
        final long due;
        Job(Runnable f, long delayMs) { this.f = f; this.due = System.currentTimeMillis() + delayMs; }
        public long getDelay(TimeUnit u) { return u.convert(due - System.currentTimeMillis(), TimeUnit.MILLISECONDS); }
        public int compareTo(Delayed o) { return Long.compare(due, ((Job) o).due); }
    }

    static final DelayQueue<Job> queue = new DelayQueue<>();

    static void start() {
        Thread t = new Thread(() -> {
            try {
                while (true) queue.take().f.run();
            } catch (InterruptedException ignored) {}
        });
        t.setDaemon(true);
        t.start();
    }

    static void schedule(Runnable f, int n) { queue.add(new Job(f, n)); }

    public static void main(String[] args) throws InterruptedException {
        start();
        schedule(() -> System.out.println("Executed after delay!"), 100);
        Thread.sleep(200); // let the job fire before exit
    }
}
