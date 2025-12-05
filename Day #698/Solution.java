// Day 698: Count regions a grid of '/'/'\\'/' ' is divided into.
// Approach: split each cell into 4 triangles (top,right,bottom,left) and union by
// the slash type plus across neighbors (Union-Find). Time O(R*C*a), Space O(R*C).
import java.util.*;

public class Solution {
    static int[] p;
    static int find(int x) { while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; } return x; }
    static void uni(int a, int b) { p[find(a)] = find(b); }

    static int regions(String[] in) {
        int R = in.length, C = 0;
        for (String s : in) C = Math.max(C, s.length());
        char[][] g = new char[R][C];
        for (int r = 0; r < R; r++) {
            Arrays.fill(g[r], ' ');
            for (int c = 0; c < in[r].length(); c++) g[r][c] = in[r].charAt(c);
        }
        p = new int[R * C * 4];
        for (int i = 0; i < p.length; i++) p[i] = i;
        for (int r = 0; r < R; r++)
            for (int c = 0; c < C; c++) {
                int base = (r * C + c) * 4; // 0=T,1=R,2=B,3=L
                char ch = g[r][c];
                if (ch == '/') { uni(base, base + 3); uni(base + 1, base + 2); }
                else if (ch == '\\') { uni(base, base + 1); uni(base + 2, base + 3); }
                else { uni(base, base + 1); uni(base + 1, base + 2); uni(base + 2, base + 3); }
                if (c + 1 < C) uni(base + 1, ((r * C + c + 1) * 4) + 3);
                if (r + 1 < R) uni(base + 2, (((r + 1) * C + c) * 4));
            }
        int cnt = 0;
        for (int i = 0; i < p.length; i++) if (find(i) == i) cnt++;
        return cnt;
    }

    public static void main(String[] args) {
        String[] grid = {"\\    /", " \\  /", "  \\/"};
        System.out.println(regions(grid)); // 3
    }
}
