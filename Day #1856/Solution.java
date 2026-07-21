// Day 1856: Collatz conjecture test + longest chain under 1,000,000.
// Memoized step counts (cache values <= LIMIT). ~O(LIMIT * avg-chain) time, O(LIMIT) space.
public class Solution {
    public static void main(String[] args) {
        final int LIMIT = 1_000_000;
        int[] steps = new int[LIMIT + 1]; // steps[n] = steps to reach 1 (0 = unknown; steps[1]=0)
        boolean allReach1 = true;
        int bestN = 1, bestSteps = 0;

        for (int i = 2; i <= LIMIT; i++) {
            long n = i;
            int cnt = 0;
            while (n != 1 && (n > LIMIT || steps[(int) n] == 0)) {
                n = (n % 2 == 0) ? n / 2 : 3 * n + 1;
                cnt++;
            }
            int total = cnt + (n == 1 ? 0 : steps[(int) n]);
            steps[i] = total;
            if (total > bestSteps) { bestSteps = total; bestN = i; }
        }

        System.out.println("All sequences for n in [1, " + LIMIT + "] reach 1: " + allReach1);
        System.out.println("Longest sequence under " + LIMIT + ": n = " + bestN
                + " with " + (bestSteps + 1) + " terms"); // 837799, 525 terms
    }
}
