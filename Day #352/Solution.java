// Validate crossword: rotational symmetry, all white runs (H & V) length>=3, white cells connected. O(N^2).
import java.util.*;

public class Solution {
    static boolean valid(String[] g) {
        int n = g.length;
        // 1. rotational symmetry
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if ((g[i].charAt(j) == '#') != (g[n-1-i].charAt(n-1-j) == '#')) return false;
        // 2a. horizontal runs >= 3
        for (int i = 0; i < n; i++) {
            int run = 0;
            for (int j = 0; j <= n; j++) {
                if (j < n && g[i].charAt(j) == '.') run++;
                else { if (run > 0 && run < 3) return false; run = 0; }
            }
        }
        // 2b. vertical runs >= 3
        for (int j = 0; j < n; j++) {
            int run = 0;
            for (int i = 0; i <= n; i++) {
                if (i < n && g[i].charAt(j) == '.') run++;
                else { if (run > 0 && run < 3) return false; run = 0; }
            }
        }
        // 3. connectivity
        List<int[]> whites = new ArrayList<>();
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (g[i].charAt(j) == '.') whites.add(new int[]{i, j});
        if (whites.isEmpty()) return true;
        boolean[][] seen = new boolean[n][n];
        Deque<int[]> q = new ArrayDeque<>();
        q.add(whites.get(0)); seen[whites.get(0)[0]][whites.get(0)[1]] = true;
        int cnt = 0;
        int[] dx = {1,-1,0,0}, dy = {0,0,1,-1};
        while (!q.isEmpty()) {
            int[] c = q.poll(); cnt++;
            for (int d = 0; d < 4; d++) {
                int ni = c[0] + dx[d], nj = c[1] + dy[d];
                if (ni >= 0 && ni < n && nj >= 0 && nj < n && g[ni].charAt(nj) == '.' && !seen[ni][nj]) {
                    seen[ni][nj] = true; q.add(new int[]{ni, nj});
                }
            }
        }
        return cnt == whites.size();
    }

    public static void main(String[] args) {
        String[] gridA = {".....", ".....", ".....", ".....", "....."};
        String[] gridB = {"#....", ".....", ".....", ".....", "....."};
        System.out.println(valid(gridA));
        System.out.println(valid(gridB));
    }
}
