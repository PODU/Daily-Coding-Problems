// Day 1179: Debounce - call f only after N ms elapse with no further invocations.
// A single-thread scheduler; each call cancels the pending task and reschedules.
// Time O(1) per call.
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;

public class Solution {
    static class Debouncer {
        private final ScheduledExecutorService sch = Executors.newSingleThreadScheduledExecutor();
        private final long delayMs;
        private final Runnable f;
        private ScheduledFuture<?> pending;
        Debouncer(long delayMs, Runnable f) { this.delayMs = delayMs; this.f = f; }
        synchronized void call() {
            if (pending != null) pending.cancel(false);
            pending = sch.schedule(f, delayMs, TimeUnit.MILLISECONDS);
        }
        void shutdown() { sch.shutdown(); }
    }

    public static void main(String[] args) throws InterruptedException {
        AtomicInteger count = new AtomicInteger(0);
        Debouncer d = new Debouncer(100, count::incrementAndGet);
        for (int i = 0; i < 5; i++) {           // rapid burst, every 20ms
            d.call();
            Thread.sleep(20);
        }
        Thread.sleep(300);
        System.out.println("f executed " + count.get() + " time(s)"); // 1
        d.shutdown();
    }
}
