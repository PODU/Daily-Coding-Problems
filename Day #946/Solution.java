// Day 946: kth row of Pascal's triangle (1-indexed) using O(k) space.
// In-place update of a single row, right-to-left. Time O(k^2), Space O(k).
import java.util.*;

public class Solution {
    static long[] pascalRow(int k) {
        long[] row = new long[k];
        row[0] = 1;
        for (int i = 1; i < k; i++)
            for (int j = i; j > 0; j--)
                row[j] += row[j - 1];
        return row;
    }

    public static void main(String[] args) {
        int k = 5;
        long[] r = pascalRow(k);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < r.length; i++) {
            if (i > 0) sb.append(' ');
            sb.append(r[i]);
        }
        System.out.println(sb.toString());
    }
}
