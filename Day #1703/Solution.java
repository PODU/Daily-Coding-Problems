// Prefix sums: precompute prefix[k]=sum(L[0:k]); sum(i,j)=prefix[j]-prefix[i].
// Preprocess O(n), query O(1), Space O(n).
public class Solution {
    static class RangeSum {
        long[] prefix;
        RangeSum(int[] L) {
            prefix = new long[L.length + 1];
            for (int k = 0; k < L.length; k++) prefix[k + 1] = prefix[k] + L[k];
        }
        long sum(int i, int j) { return prefix[j] - prefix[i]; }
    }

    public static void main(String[] args) {
        int[] L = {1, 2, 3, 4, 5};
        RangeSum rs = new RangeSum(L);
        System.out.println(rs.sum(1, 3));
    }
}
