// Approximate median: median of k random samples (seeded RNG) -> rank in [N/4, 3N/4] whp.
// Time: O(k log k), o(N) for k<N; Space: O(k).
import java.util.*;

public class Solution {
    static int approxMedian(int[] a, int k, long seed) {
        Random rng = new Random(seed);
        int[] sample = new int[k];
        for (int i = 0; i < k; i++) sample[i] = a[rng.nextInt(a.length)];
        Arrays.sort(sample);
        return sample[k / 2];
    }

    public static void main(String[] args) {
        int[] a = new int[101]; // N = 101, values 0..100
        for (int i = 0; i <= 100; i++) a[i] = i;
        int N = a.length;
        int val = approxMedian(a, 15, 42);
        int rank = val; // rank in sorted 0..100 equals value
        boolean ok = (N / 4 <= rank) && (rank <= (3 * N) / 4);
        System.out.println("Approximate median is within [N/4, 3N/4]: " + (ok ? "true" : "false"));
    }
}
