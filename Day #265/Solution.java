// Day 265: Minimum bonuses. Two-pass scan (left-to-right then right-to-left),
// each worker gets max of the two passes. Time O(n), space O(n).
import java.util.*;

public class Solution {
    static int[] bonuses(int[] lines) {
        int n = lines.length;
        int[] b = new int[n];
        Arrays.fill(b, 1);
        for (int i = 1; i < n; i++)
            if (lines[i] > lines[i - 1]) b[i] = b[i - 1] + 1;
        for (int i = n - 2; i >= 0; i--)
            if (lines[i] > lines[i + 1]) b[i] = Math.max(b[i], b[i + 1] + 1);
        return b;
    }

    public static void main(String[] args) {
        int[] lines = {10, 40, 200, 1000, 60, 30};
        int[] b = bonuses(lines);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < b.length; i++) {
            sb.append(b[i]);
            if (i + 1 < b.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
