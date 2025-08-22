// Day 152: Weighted random sampling. Build cumulative distribution, draw u in
// [0,1) and binary-search the bucket. Preprocess O(n), per-sample O(log n).
import java.util.*;

public class Solution {
    int[] nums;
    double[] cdf;
    Random rng;

    Solution(int[] nums, double[] probs, long seed) {
        this.nums = nums;
        this.cdf = new double[probs.length];
        this.rng = new Random(seed);
        double acc = 0;
        for (int i = 0; i < probs.length; i++) { acc += probs[i]; cdf[i] = acc; }
    }

    int sample() {
        double u = rng.nextDouble();
        int lo = 0, hi = cdf.length - 1;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (cdf[mid] < u) lo = mid + 1; else hi = mid;
        }
        return nums[lo];
    }

    public static void main(String[] args) {
        int[] nums = {1, 2, 3, 4};
        double[] probs = {0.1, 0.5, 0.2, 0.2};
        Solution s = new Solution(nums, probs, 42);
        int N = 1000000;
        Map<Integer,Integer> cnt = new HashMap<>();
        for (int i = 0; i < N; i++)
            cnt.merge(s.sample(), 1, Integer::sum);
        for (int n : nums)
            System.out.printf("%d: %.2f%%%n", n, 100.0 * cnt.getOrDefault(n, 0) / N);
    }
}
