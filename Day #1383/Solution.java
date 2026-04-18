// Weighted sampling: build cumulative prefix array, draw u in [0,1), binary search to pick. O(n) prep, O(log n) per sample.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] numbers = {1, 2, 3, 4};
        double[] probs = {0.1, 0.5, 0.2, 0.2};
        int n = numbers.length;

        double[] cum = new double[n];
        double acc = 0;
        for (int i = 0; i < n; i++) { acc += probs[i]; cum[i] = acc; }

        Random rng = new Random(42);
        final int N = 100000;
        long[] counts = new long[n];
        for (int s = 0; s < N; s++) {
            double u = rng.nextDouble();
            int lo = 0, hi = n - 1;
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (cum[mid] < u) lo = mid + 1;
                else hi = mid;
            }
            counts[lo]++;
        }

        for (int i = 0; i < n; i++)
            System.out.printf("%d: %.2f%n", numbers[i], (double) counts[i] / N);
    }
}
