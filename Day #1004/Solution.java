// Day 1004: Range sum query sum(i, j) = L[i:j].
// Pre-process a prefix-sum array (O(N)); each query is prefix[j]-prefix[i] in O(1).
public class Solution {
    static class RangeSum {
        long[] prefix;
        RangeSum(long[] L) {
            prefix = new long[L.length + 1];
            for (int i = 0; i < L.length; i++) prefix[i + 1] = prefix[i] + L[i];
        }
        long sum(int i, int j) { return prefix[j] - prefix[i]; }
    }

    public static void main(String[] args) {
        RangeSum rs = new RangeSum(new long[]{1, 2, 3, 4, 5});
        System.out.println(rs.sum(1, 3)); // 5
    }
}
