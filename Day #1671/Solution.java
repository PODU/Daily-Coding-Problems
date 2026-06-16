// Day 1671: Min columns to remove so each column is non-decreasing top->bottom.
// Count columns containing any out-of-order adjacent pair. Time O(N*M), Space O(1).
public class Solution {
    static int minDeletions(String[] g) {
        if (g.length == 0) return 0;
        int rows = g.length, cols = g[0].length(), del = 0;
        for (int j = 0; j < cols; j++)
            for (int i = 0; i + 1 < rows; i++)
                if (g[i].charAt(j) > g[i + 1].charAt(j)) { del++; break; }
        return del;
    }

    public static void main(String[] args) {
        String[] grid = {"cba", "daf", "ghi"};
        System.out.println(minDeletions(grid)); // 1
    }
}
