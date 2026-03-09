// Day 1182: Split N into k contiguous partitions minimizing the maximum sum.
// Binary search the answer in [max element, total]; greedy feasibility check.
// Time O(N log(sum)), Space O(1).
public class Solution {
    static boolean feasible(int[] a, int k, long cap) {
        long cur = 0; int parts = 1;
        for (int x : a) {
            if (cur + x > cap) { parts++; cur = x; if (parts > k) return false; }
            else cur += x;
        }
        return true;
    }

    static long splitArray(int[] a, int k) {
        long lo = 0, hi = 0;
        for (int x : a) { lo = Math.max(lo, x); hi += x; }
        while (lo < hi) {
            long mid = lo + (hi - lo) / 2;
            if (feasible(a, k, mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    public static void main(String[] args) {
        System.out.println(splitArray(new int[]{5, 1, 2, 7, 3, 4}, 3)); // 8
    }
}
