// Day 1499: Job scheduler that calls f after n ms using java.util.Timer.
// Time O(1) to schedule, Space O(1).
import java.util.Timer;
import java.util.TimerTask;
import java.util.concurrent.CountDownLatch;

public class Solution {
    static void schedule(Runnable f, int n) {
        Timer timer = new Timer();
        timer.schedule(new TimerTask() {
            public void run() {
                f.run();
                timer.cancel();
            }
        }, n);
    }

    public static void main(String[] args) throws InterruptedException {
        CountDownLatch latch = new CountDownLatch(1);
        schedule(() -> {
            System.out.println("Job executed after delay");
            latch.countDown();
        }, 100);
        latch.await(); // wait for the scheduled job before exiting
    }
}
