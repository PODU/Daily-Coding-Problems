// Day 840: Print a string in zigzag form across k lines.
// Char i sits at column i; its row follows the zigzag 0,1,..,k-1,k-2,..,1,0,...
// Build k rows of spaces, place each char, print with trailing spaces trimmed. Time O(N*k).
public class Solution {
    static String zigzag(String s, int k) {
        if (k <= 0) return "";
        if (k == 1) return s;
        int n = s.length();
        char[][] rows = new char[k][n];
        for (char[] r : rows) java.util.Arrays.fill(r, ' ');
        int row = 0, step = 1;
        for (int i = 0; i < n; i++) {
            rows[row][i] = s.charAt(i);
            if (row == 0) step = 1;
            else if (row == k - 1) step = -1;
            row += step;
        }
        StringBuilder out = new StringBuilder();
        for (int r = 0; r < k; r++) {
            String line = new String(rows[r]);
            int end = line.length();
            while (end > 0 && line.charAt(end - 1) == ' ') end--;
            out.append(line, 0, end);
            if (r != k - 1) out.append("\n");
        }
        return out.toString();
    }

    public static void main(String[] args) {
        System.out.println(zigzag("thisisazigzag", 4));
    }
}
