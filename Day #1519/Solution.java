// Two dice games via Monte Carlo simulation. Expected rolls: "5 then 6"=36, "5 then 5"=42.
// Time: O(trials * rolls_per_trial). Space: O(1).
import java.util.Random;

public class Solution {
    static double simulate(int first, int second, long trials, Random rng) {
        long total = 0;
        for (long t = 0; t < trials; t++) {
            int prev = 0, rolls = 0;
            while (true) {
                int r = rng.nextInt(6) + 1;
                rolls++;
                if (prev == first && r == second) break;
                prev = r;
            }
            total += rolls;
        }
        return (double) total / trials;
    }

    public static void main(String[] args) {
        Random rng = new Random(12345);
        long trials = 500000;
        double e1 = simulate(5, 6, trials, rng);
        double e2 = simulate(5, 5, trials, rng);
        System.out.printf("Game 1 (five then six) expected rolls: %.2f%n", e1);
        System.out.printf("Game 2 (five then five) expected rolls: %.2f%n", e2);
        System.out.println("Alice should play: Game 1 (five then six)");
    }
}
