// Day 1728: Simulate a fair coin from a biased one (Von Neumann trick).
// Toss biased coin twice; (0,1)->0, (1,0)->1, else retry. Expected P(heads) ~= 0.5.
// Time: O(1) expected tosses per fair() call. Space: O(1).
import java.util.Random;

public class Solution {
    static final Random rng = new Random(12345);

    // Biased coin: returns 1 with probability p (= 0.3), else 0.
    static int tossBiased() {
        return rng.nextDouble() < 0.3 ? 1 : 0;
    }

    // Von Neumann: extract a fair bit from the biased coin.
    static int fair() {
        while (true) {
            int a = tossBiased();
            int b = tossBiased();
            if (a == 0 && b == 1) return 0;
            if (a == 1 && b == 0) return 1;
        }
    }

    public static void main(String[] args) {
        final int N = 100000;
        long heads = 0;
        for (int i = 0; i < N; i++) heads += fair();
        double ratio = (double) heads / N;
        System.out.printf("Fair coin over %d tosses, P(heads) ~= %.2f%n", N, ratio);
    }
}
