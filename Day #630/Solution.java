// Job scheduler: run f after n ms using ScheduledExecutorService.schedule.
// Background thread fires once after the delay; main awaits completion then exits.
import java.util.concurrent.*;

public class Solution {
    static ScheduledFuture<?> schedule(Runnable f, int n, ScheduledExecutorService exec) {
        return exec.schedule(f, n, TimeUnit.MILLISECONDS);
    }

    public static void main(String[] args) throws Exception {
        ScheduledExecutorService exec = Executors.newSingleThreadScheduledExecutor();
        ScheduledFuture<?> future = schedule(
            () -> System.out.println("Job executed after 100 ms"), 100, exec);
        future.get(); // wait for the job to run before exiting
        exec.shutdown();
        System.out.println("Main: job completed, exiting");
    }
}
