// Debounce: wrap f so it runs only after N ms of silence; each call resets the timer.
// Uses a single-thread ScheduledExecutorService, cancelling the pending task each call.
// Time: O(1) per call, Space: O(1).
import java.util.concurrent.*;
import java.util.function.Consumer;

public class Solution {
    static <T> Consumer<T> debounce(Consumer<T> f, long n, ScheduledExecutorService exec) {
        final ScheduledFuture<?>[] pending = new ScheduledFuture<?>[1];
        return (T arg) -> {
            synchronized (pending) {
                if (pending[0] != null) pending[0].cancel(false);
                pending[0] = exec.schedule(() -> f.accept(arg), n, TimeUnit.MILLISECONDS);
            }
        };
    }

    public static void main(String[] args) throws InterruptedException {
        ScheduledExecutorService exec = Executors.newSingleThreadScheduledExecutor();
        final int[] calls = {0};

        Consumer<Integer> debounced = debounce((Integer x) -> {
            calls[0]++;
            System.out.println("f called with " + x + "; total calls = " + calls[0]);
        }, 100, exec);

        for (int i = 1; i <= 5; i++) debounced.accept(i); // burst of 5 calls

        Thread.sleep(300); // wait > N ms
        System.out.println("done; f ran " + calls[0] + " time(s)");
        exec.shutdown();
    }
}
