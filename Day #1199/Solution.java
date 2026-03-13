// Job scheduler: call f after n ms using java.util.Timer (one-shot background timer).
// Time: O(1) to schedule; Space: O(1). Main sleeps so the job runs before exit.
import java.util.Timer;
import java.util.TimerTask;

public class Solution {
    static void schedule(Runnable f, int n) {
        Timer timer = new Timer();
        timer.schedule(new TimerTask() {
            public void run() {
                f.run();
                timer.cancel(); // release the timer thread after firing
            }
        }, n);
    }

    public static void main(String[] args) throws InterruptedException {
        System.out.println("Scheduling job...");
        schedule(() -> System.out.println("Job executed after 100 ms"), 100);
        // Wait long enough for the scheduled job to fire before the program exits.
        Thread.sleep(200);
    }
}
