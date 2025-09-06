// Day 224: Smallest positive integer not expressible as a subset sum (sorted array).
// Approach: greedy. Keep reachable range [1, ans-1]; if next a <= ans, extend by a, else ans is the gap.
// Time O(N), Space O(1).
public class Solution {
    static long smallestNonSubsetSum(long[] a) {
        long ans = 1; // smallest unreachable so far
        for (long x : a) {
            if (x > ans) break;
            ans += x;
        }
        return ans;
    }

    public static void main(String[] args) {
        System.out.println(smallestNonSubsetSum(new long[]{1, 2, 3, 10})); // 7
    }
}
