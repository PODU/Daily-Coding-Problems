// Day 105: Debounce. Each call bumps a generation counter and schedules f after N
// ms; f runs only if its generation is still the latest. O(1) per call.
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;

public class Solution {
    static Runnable debounce(Runnable f, long ms, ScheduledExecutorService ex) {
        AtomicLong gen = new AtomicLong();
        return () -> {
            long my = gen.incrementAndGet();
            ex.schedule(() -> { if (gen.get() == my) f.run(); }, ms, TimeUnit.MILLISECONDS);
        };
    }

    public static void main(String[] args) throws InterruptedException {
        ScheduledExecutorService ex = Executors.newSingleThreadScheduledExecutor();
        AtomicInteger count = new AtomicInteger();
        Runnable d = debounce(() -> { count.incrementAndGet(); System.out.println("f called"); },
                              100, ex);
        d.run(); d.run(); d.run();                 // 3 rapid calls
        Thread.sleep(300);
        System.out.println("Total calls to f: " + count.get()); // 1
        ex.shutdown();
    }
}
