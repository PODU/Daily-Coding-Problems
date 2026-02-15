// Job scheduler: schedule f after n ms using ScheduledExecutorService. O(1) schedule, waits for completion.
import java.util.concurrent.*;

public class Solution {
    static void schedule(Runnable f, int delayMs) {
        ScheduledExecutorService ses = Executors.newSingleThreadScheduledExecutor();
        ses.schedule(() -> {
            f.run();
            ses.shutdown();
        }, delayMs, TimeUnit.MILLISECONDS);
    }

    public static void main(String[] args) throws InterruptedException {
        System.out.println("Scheduling job...");
        CountDownLatch latch = new CountDownLatch(1);
        schedule(() -> {
            System.out.println("Job executed!");
            latch.countDown();
        }, 100);
        latch.await();
    }
}
