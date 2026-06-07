// Day 1627: Debounce f by N ms. Each call resets an N-ms timer; f fires
// only after a quiet gap of N ms. Time O(1) per call.
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;

public class Solution {
    static class Debouncer {
        private final ScheduledExecutorService sched = Executors.newSingleThreadScheduledExecutor();
        private final long delayMs;
        private final Runnable f;
        private ScheduledFuture<?> future;

        Debouncer(Runnable f, long delayMs) { this.f = f; this.delayMs = delayMs; }

        synchronized void call() {
            if (future != null) future.cancel(false);
            future = sched.schedule(f, delayMs, TimeUnit.MILLISECONDS);
        }

        void shutdown() { sched.shutdown(); }
    }

    public static void main(String[] args) throws InterruptedException {
        AtomicInteger calls = new AtomicInteger(0);
        Debouncer d = new Debouncer(calls::incrementAndGet, 100);
        d.call(); d.call(); d.call();
        Thread.sleep(200);
        System.out.println("f was called " + calls.get() + " time(s)");
        d.shutdown();
    }
}
