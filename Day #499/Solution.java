// Validate American-style crossword grid: every white cell in a horizontal AND vertical run
// of length >=3, single connected component of white cells, 180-degree rotational symmetry.
// Time: O(N^2), Space: O(N^2).
import java.util.ArrayDeque;

public class Solution {
    static boolean isCrossword(String[] g) {
        int n = g.length;
        if (n == 0) return false;

        // Rule 4: rotational symmetry.
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (g[i].charAt(j) != g[n-1-i].charAt(n-1-j)) return false;

        // Rules 1 & 2: runs of length >= 3 in both directions.
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (g[i].charAt(j) != '.') continue;
                int l = j; while (l > 0 && g[i].charAt(l-1) == '.') l--;
                int r = j; while (r < n-1 && g[i].charAt(r+1) == '.') r++;
                if (r - l + 1 < 3) return false;
                int t = i; while (t > 0 && g[t-1].charAt(j) == '.') t--;
                int b = i; while (b < n-1 && g[b+1].charAt(j) == '.') b++;
                if (b - t + 1 < 3) return false;
            }
        }

        // Rule 3: connectivity via BFS.
        int total = 0, startR = -1, startC = -1;
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (g[i].charAt(j) == '.') {
                    total++;
                    if (startR < 0) { startR = i; startC = j; }
                }
        if (total == 0) return true;
        boolean[][] seen = new boolean[n][n];
        ArrayDeque<int[]> q = new ArrayDeque<>();
        q.add(new int[]{startR, startC});
        seen[startR][startC] = true;
        int cnt = 0;
        int[] dx = {0,0,1,-1}, dy = {1,-1,0,0};
        while (!q.isEmpty()) {
            int[] cur = q.poll(); cnt++;
            for (int d = 0; d < 4; d++) {
                int nx = cur[0]+dx[d], ny = cur[1]+dy[d];
                if (nx>=0&&nx<n&&ny>=0&&ny<n&&g[nx].charAt(ny)=='.'&&!seen[nx][ny]) {
                    seen[nx][ny] = true; q.add(new int[]{nx, ny});
                }
            }
        }
        return cnt == total;
    }

    public static void main(String[] args) {
        String[] valid = {".....", ".....", ".....", ".....", "....."};
        String[] invalid = {"..#..", ".....", "#...#", ".....", "..#.."};
        System.out.println(isCrossword(valid) ? "true" : "false");
        System.out.println(isCrossword(invalid) ? "true" : "false");
    }
}
