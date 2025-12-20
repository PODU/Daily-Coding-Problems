// Day 769: Expected rolls for "5 then 6" vs "5 then 5" stopping games.
// Exact via 2-state Markov chains (E1=36, E2=42) plus Monte-Carlo check. O(trials).
import java.util.Random;

public class Solution {
    static double simulate(int second, long trials, Random rng) {
        long total = 0;
        for (long t = 0; t < trials; t++) {
            int prev = 0, rolls = 0;
            while (true) {
                int r = rng.nextInt(6) + 1; rolls++;
                if (prev == 5 && r == second) break;
                prev = r;
            }
            total += rolls;
        }
        return (double) total / trials;
    }

    public static void main(String[] args) {
        Random rng = new Random(12345);
        long trials = 500000;
        System.out.printf("Game 1 (5 then 6): exact=36, simulated=%.2f%n", simulate(6, trials, rng));
        System.out.printf("Game 2 (5 then 5): exact=42, simulated=%.2f%n", simulate(5, trials, rng));
        System.out.println("Alice should play Game 1 (5 then 6); it has the lower expected cost.");
    }
}
