// Day 1296: Smallest positive integer not expressible as a subset sum of a sorted array.
// Greedy: track reachable range [1..r]; if next a[i] <= r+1 extend, else answer r+1. O(N) time.
public class Solution {
    static long smallestNonSubsetSum(long[] a) {
        long r = 0;
        for (long x : a) {
            if (x > r + 1) break;
            r += x;
        }
        return r + 1;
    }

    public static void main(String[] args) {
        long[] a = {1, 2, 3, 10};
        System.out.println(smallestNonSubsetSum(a)); // 7
    }
}
