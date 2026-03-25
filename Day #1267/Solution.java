// Day 1267: Range sum query with preprocessing.
// Prefix-sum array: O(n) preprocess, O(1) per sum(i, j) query.
public class Solution {
    static class RangeSum {
        long[] prefix;
        RangeSum(int[] L) {
            prefix = new long[L.length + 1];
            for (int i = 0; i < L.length; i++) prefix[i + 1] = prefix[i] + L[i];
        }
        long sum(int i, int j) { return prefix[j] - prefix[i]; } // L[i:j]
    }

    public static void main(String[] args) {
        RangeSum rs = new RangeSum(new int[]{1, 2, 3, 4, 5});
        System.out.println(rs.sum(1, 3));
    }
}
