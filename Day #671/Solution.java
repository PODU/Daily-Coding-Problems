// Day 671: Debounce. Each call bumps a generation id and schedules f after N ms;
// f fires only if no newer call arrived. Per-call O(1).
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicLong;
import java.util.function.Consumer;

public class Solution {
    static class Debouncer {
        final AtomicLong gen = new AtomicLong(0);
        final long n;
        final Consumer<String> f;
        final ScheduledExecutorService es = Executors.newSingleThreadScheduledExecutor();
        Debouncer(long n, Consumer<String> f) { this.n = n; this.f = f; }
        void call(String arg) {
            long id = gen.incrementAndGet();
            es.schedule(() -> { if (gen.get() == id) f.accept(arg); }, n, TimeUnit.MILLISECONDS);
        }
    }

    public static void main(String[] args) throws InterruptedException {
        Debouncer d = new Debouncer(50, s -> System.out.println("f called with: " + s));
        for (String s : new String[]{"a", "b", "c", "d", "e"}) d.call(s);
        Thread.sleep(200);
        d.es.shutdown(); // prints once: f called with: e
    }
}
