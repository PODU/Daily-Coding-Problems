// Day 493: Sample from a discrete distribution.
// Precompute cumulative probabilities; binary-search a uniform r in [0,1).
// Time: O(n) preprocessing, O(log n) per sample. Space: O(n).
import java.util.*;

public class Solution {
    static class DiscreteSampler {
        int[] numbers;
        double[] cdf;
        DiscreteSampler(int[] nums, double[] probs) {
            numbers = nums;
            cdf = new double[nums.length];
            double acc = 0;
            for (int i = 0; i < probs.length; i++) { acc += probs[i]; cdf[i] = acc; }
        }
        int sample(double r) {
            int lo = 0, hi = cdf.length - 1;
            while (lo < hi) {
                int mid = (lo + hi) / 2;
                if (cdf[mid] > r) hi = mid; else lo = mid + 1;
            }
            return numbers[lo];
        }
    }

    public static void main(String[] args) {
        int[] numbers = {1, 2, 3, 4};
        double[] probs = {0.1, 0.5, 0.2, 0.2};
        DiscreteSampler s = new DiscreteSampler(numbers, probs);

        Random rng = new Random(42);
        int N = 100000;
        Map<Integer, Integer> counts = new HashMap<>();
        for (int i = 0; i < N; i++) {
            int v = s.sample(rng.nextDouble());
            counts.merge(v, 1, Integer::sum);
        }
        for (int n : numbers)
            System.out.printf("%d: %.3f%n", n, counts.getOrDefault(n, 0) / (double) N);
    }
}
