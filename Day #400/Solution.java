// Prefix-sum array P (P[k]=sum of first k elems); sum(i,j)=P[j]-P[i]. Preprocess O(n), query O(1).
public class Solution {
    static class RangeSum {
        long[] P;
        RangeSum(int[] L) {
            P = new long[L.length + 1];
            for (int k = 0; k < L.length; k++) P[k + 1] = P[k] + L[k];
        }
        long sum(int i, int j) { return P[j] - P[i]; }
    }

    public static void main(String[] args) {
        int[] L = {1, 2, 3, 4, 5};
        RangeSum rs = new RangeSum(L);
        System.out.println(rs.sum(1, 3));
    }
}
