// Day 1119 - Split array into k contiguous partitions minimizing the max sum
// Binary search the answer; greedily count partitions. Time: O(n log(sum)).
public class Solution {
    static int partitionsNeeded(int[] N, long limit) {
        int count = 1;
        long cur = 0;
        for (int x : N) {
            if (cur + x > limit) { count++; cur = x; }
            else cur += x;
        }
        return count;
    }

    static long splitMinMax(int[] N, int k) {
        long lo = 0, hi = 0;
        for (int x : N) { lo = Math.max(lo, x); hi += x; }
        while (lo < hi) {
            long mid = (lo + hi) / 2;
            if (partitionsNeeded(N, mid) <= k) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    public static void main(String[] args) {
        int[] N = {5, 1, 2, 7, 3, 4};
        int k = 3;
        System.out.println(splitMinMax(N, k)); // 8
    }
}
