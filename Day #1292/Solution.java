// Day 1292: kth (0-indexed) row of Pascal's triangle.
// Update row in place from right to left. O(k^2) time, O(k) space.
import java.util.*;

public class Solution {
    static long[] pascalRow(int k) {
        long[] row = new long[k + 1];
        Arrays.fill(row, 1);
        for (int i = 1; i <= k; i++)
            for (int j = i - 1; j >= 1; j--)
                row[j] += row[j - 1];
        return row;
    }

    public static void main(String[] args) {
        long[] r = pascalRow(4); // 1 4 6 4 1
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < r.length; i++) {
            if (i > 0) sb.append(' ');
            sb.append(r[i]);
        }
        System.out.println(sb);
    }
}
