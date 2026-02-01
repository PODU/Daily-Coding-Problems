// Day 1002: Smallest positive integer not expressible as a subset sum (sorted array).
// If the next element x <= res (smallest unreachable, init 1) extend to res+x,
// else res is the answer. O(N) time, O(1) space.
public class Solution {
    static long smallestNonSubsetSum(long[] nums) {
        long res = 1;
        for (long x : nums) {
            if (x > res) break;
            res += x;
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(smallestNonSubsetSum(new long[]{1, 2, 3, 10})); // 7
    }
}
