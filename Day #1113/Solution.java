// Day 1113 - Validate an American-style crossword grid ('#' black, '.' white)
// Approach: 180-degree symmetry, every white cell in across & down run >= 3,
// and white connectivity. Time: O(N^2), Space: O(N^2).
import java.util.*;

public class Solution {
    static boolean isCrossword(String[] grid) {
        int n = grid.length;
        if (n == 0) return false;
        for (String r : grid) if (r.length() != n) return false;

        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if ((grid[i].charAt(j) == '#') != (grid[n-1-i].charAt(n-1-j) == '#'))
                    return false;

        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (grid[i].charAt(j) == '.') {
                    int len = 1, k = j - 1;
                    while (k >= 0 && grid[i].charAt(k) == '.') { len++; k--; }
                    k = j + 1;
                    while (k < n && grid[i].charAt(k) == '.') { len++; k++; }
                    if (len < 3) return false;
                    len = 1; k = i - 1;
                    while (k >= 0 && grid[k].charAt(j) == '.') { len++; k--; }
                    k = i + 1;
                    while (k < n && grid[k].charAt(j) == '.') { len++; k++; }
                    if (len < 3) return false;
                }

        List<int[]> whites = new ArrayList<>();
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (grid[i].charAt(j) == '.') whites.add(new int[]{i, j});
        if (whites.isEmpty()) return true;

        boolean[][] seen = new boolean[n][n];
        Deque<int[]> st = new ArrayDeque<>();
        st.push(whites.get(0));
        seen[whites.get(0)[0]][whites.get(0)[1]] = true;
        int cnt = 1;
        int[] dx = {1,-1,0,0}, dy = {0,0,1,-1};
        while (!st.isEmpty()) {
            int[] c = st.pop();
            for (int d = 0; d < 4; d++) {
                int ni = c[0] + dx[d], nj = c[1] + dy[d];
                if (ni>=0&&ni<n&&nj>=0&&nj<n&&grid[ni].charAt(nj)=='.'&&!seen[ni][nj]) {
                    seen[ni][nj] = true; cnt++; st.push(new int[]{ni, nj});
                }
            }
        }
        return cnt == whites.size();
    }

    public static void main(String[] args) {
        String[] valid = {"...##", ".....", ".....", ".....", "##..."};
        String[] invalid = {".....", ".....", ".....", ".....", "....#"};
        System.out.println("Grid 1 valid: " + (isCrossword(valid) ? "True" : "False"));
        System.out.println("Grid 2 valid: " + (isCrossword(invalid) ? "True" : "False"));
    }
}
