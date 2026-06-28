// Day 1731: kth row of Pascal's triangle (1-indexed) using O(k) space.
// Binomial coefficients built in place. Time O(k), Space O(k).
public class Solution {
    static long[] pascalRow(int k) {
        long[] row = new long[k];
        row[0] = 1;
        for (int i = 1; i < k; i++) row[i] = row[i - 1] * (k - i) / i;
        return row;
    }

    public static void main(String[] args) {
        int k = 5; // row [1,4,6,4,1]
        long[] r = pascalRow(k);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < r.length; i++) {
            if (i > 0) sb.append(' ');
            sb.append(r[i]);
        }
        System.out.println(sb);
    }
}
