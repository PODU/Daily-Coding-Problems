// Range sum via prefix sums: O(n) preprocessing, O(1) per query. sum(i,j) = pre[j]-pre[i].

public class Solution {
    static class RangeSum {
        long[] pre;
        RangeSum(int[] L) {
            pre = new long[L.length + 1];
            for (int k = 0; k < L.length; k++) pre[k + 1] = pre[k] + L[k];
        }
        long sum(int i, int j) { return pre[j] - pre[i]; }
    }

    public static void main(String[] args) {
        int[] L = {1, 2, 3, 4, 5};
        RangeSum rs = new RangeSum(L);
        System.out.println(rs.sum(1, 3));
    }
}
