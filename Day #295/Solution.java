// Kth row of Pascal's triangle (1-indexed) via iterative binomials in one array. O(k) space, O(k) time.
import java.util.*;

public class Solution {
    static long[] pascalRow(int k) {
        int n = k - 1; // 0-indexed row number
        long[] row = new long[k];
        row[0] = 1;
        for (int r = 1; r < k; r++) row[r] = row[r-1] * (n - r + 1) / r;
        return row;
    }

    public static void main(String[] args) {
        long[] row = pascalRow(5);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < row.length; i++) { sb.append(row[i]); if (i+1<row.length) sb.append(", "); }
        sb.append("]");
        System.out.println(sb);
    }
}
