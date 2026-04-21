// Monte-Carlo simulation of two stop conditions plus exact theory.
// E[rolls until 5->6] = 36 (distinct faces); E[rolls until 5->5] = 42 (same face).
// Time O(trials * rolls), Space O(1).
import java.util.Random;

public class Solution {
    static long play(int second, Random rng) {
        long rolls = 0; int prev = 0;
        while (true) {
            int cur = rng.nextInt(6) + 1;
            rolls++;
            if (prev == 5 && cur == second) return rolls;
            prev = cur;
        }
    }

    public static void main(String[] args) {
        Random rng = new Random(42);
        long trials = 200000, s56 = 0, s55 = 0;
        for (long i = 0; i < trials; i++) s56 += play(6, rng);
        for (long i = 0; i < trials; i++) s55 += play(5, rng);
        System.out.printf("Game 1 (five then six): simulated avg = %.2f, theoretical = 36%n",
                (double) s56 / trials);
        System.out.printf("Game 2 (five then five): simulated avg = %.2f, theoretical = 42%n",
                (double) s55 / trials);
        System.out.println("Alice should play Game 1 (five-then-six): fewer expected rolls, less pay.");
    }
}
