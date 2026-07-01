// Day 1746: Weighted random sampler.
// Approach: prefix-sum (CDF) of probabilities + binary search on uniform U[0,1).
// Build O(n), sample O(log n) time, O(n) space.
import java.util.*;

public class Solution {
    static int[] nums;
    static double[] cdf;
    static Random rng = new Random(42);

    static void build(int[] n, double[] p) {
        nums = n;
        cdf = new double[p.length];
        double acc = 0;
        for (int i = 0; i < p.length; i++) { acc += p[i]; cdf[i] = acc; }
    }
    static int sample() {
        double r = rng.nextDouble();
        int lo = 0, hi = cdf.length - 1;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (cdf[mid] < r) lo = mid + 1; else hi = mid;
        }
        return nums[lo];
    }

    public static void main(String[] args) {
        int[] numbers = {1, 2, 3, 4};
        double[] probs = {0.1, 0.5, 0.2, 0.2};
        build(numbers, probs);

        int N = 1000000;
        Map<Integer,Integer> cnt = new HashMap<>();
        for (int i = 0; i < N; i++) cnt.merge(sample(), 1, Integer::sum);

        System.out.println("Observed frequencies over " + N + " samples:");
        for (int x : numbers)
            System.out.printf("%d: %.1f%%%n", x, 100.0 * cnt.getOrDefault(x,0) / N);
        System.out.println("Expected: 1 10% of the time, 2 50% of the time, and 3 and 4 20% of the time");
    }
}
