// Split array into k contiguous partitions minimizing the max partition sum.
// Binary search on the answer + greedy feasibility. Time: O(n log(sum)). Space: O(1).
public class Solution {
    static boolean feasible(int[] a, int k, long cap) {
        long cur = 0; int parts = 1;
        for (int x : a) {
            if (x > cap) return false;
            if (cur + x > cap) { parts++; cur = x; }
            else cur += x;
        }
        return parts <= k;
    }

    static long splitArray(int[] a, int k) {
        long lo = 0, hi = 0;
        for (int x : a) { lo = Math.max(lo, x); hi += x; }
        while (lo < hi) {
            long mid = lo + (hi - lo) / 2;
            if (feasible(a, k, mid)) hi = mid; else lo = mid + 1;
        }
        return lo;
    }

    public static void main(String[] args) {
        int[] N = {5, 1, 2, 7, 3, 4};
        System.out.println(splitArray(N, 3)); // 8
    }
}
