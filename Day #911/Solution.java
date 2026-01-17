// Reservoir sampling (size 1): replace pick with prob 1/i for i-th element. O(n) time, O(1) space.
import java.util.Random;

public class Solution {
    static int reservoirSample(int[] stream, Random rng) {
        int pick = 0, i = 0;
        for (int x : stream) {
            i++; // 1-indexed
            if (rng.nextInt(i) == 0) pick = x; // prob 1/i
        }
        return pick;
    }

    public static void main(String[] args) {
        Random rng = new Random(12345);
        int n = 10;
        int[] stream = new int[n];
        for (int k = 0; k < n; k++) stream[k] = k; // 0..9

        System.out.println("Sampled element: " + reservoirSample(stream, rng));

        int trials = 100000;
        int[] freq = new int[n];
        for (int t = 0; t < trials; t++) freq[reservoirSample(stream, rng)]++;
        System.out.printf("Approx frequencies over %d trials (expect ~%.4f each):%n",
                trials, 1.0 / n);
        for (int v = 0; v < n; v++)
            System.out.printf("  %d: %.4f%n", v, (double) freq[v] / trials);
        System.out.println("Distribution is ~uniform.");
    }
}
