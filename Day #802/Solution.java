// Day 802: Sample a number per given probability distribution.
// Build CDF prefix sums once O(n); each sample draws u~U(0,1) + binary search O(log n).
import java.util.*;

public class Solution {
    int[] nums;
    double[] cdf;
    Random rng;

    Solution(int[] nums, double[] probs, long seed) {
        this.nums = nums;
        this.cdf = new double[probs.length];
        double acc = 0;
        for (int i = 0; i < probs.length; i++) { acc += probs[i]; cdf[i] = acc; }
        this.rng = new Random(seed);
    }

    int sample() {
        double u = rng.nextDouble();
        int lo = 0, hi = cdf.length - 1;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (cdf[mid] < u) lo = mid + 1; else hi = mid;
        }
        return nums[lo];
    }

    public static void main(String[] args) {
        int[] numbers = {1, 2, 3, 4};
        double[] probs = {0.1, 0.5, 0.2, 0.2};
        Solution s = new Solution(numbers, probs, 42);
        Map<Integer, Integer> count = new TreeMap<>();
        int trials = 100000;
        for (int i = 0; i < trials; i++)
            count.merge(s.sample(), 1, Integer::sum);
        for (int n : numbers)
            System.out.printf("%d: %.2f%n", n, count.getOrDefault(n, 0) / (double) trials);
        // ~ 1:0.10  2:0.50  3:0.20  4:0.20
    }
}
