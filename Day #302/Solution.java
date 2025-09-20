// Day 302: Count regions divided by slashes. Split each cell into 4 triangles,
// union-find adjacent triangles. Time O(N*M*alpha), space O(N*M).
import java.util.*;
public class Solution {
    static int[] p;
    static int f(int x) { while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; } return x; }
    static void u(int a, int b) { p[f(a)] = f(b); }
    static int countRegions(String[] in) {
        int n = in.length, M = 0;
        for (String s : in) M = Math.max(M, s.length());
        String[] g = new String[n];
        for (int i = 0; i < n; i++) {
            StringBuilder sb = new StringBuilder(in[i]);
            while (sb.length() < M) sb.append(' ');
            g[i] = sb.toString();
        }
        p = new int[n * M * 4];
        for (int i = 0; i < p.length; i++) p[i] = i;
        for (int i = 0; i < n; i++) for (int j = 0; j < M; j++) {
            char c = g[i].charAt(j);
            int base = (i * M + j) * 4;
            if (c == '/') { u(base + 0, base + 3); u(base + 1, base + 2); }
            else if (c == '\\') { u(base + 0, base + 1); u(base + 2, base + 3); }
            else { u(base + 0, base + 1); u(base + 1, base + 2); u(base + 2, base + 3); }
            if (j + 1 < M) u(base + 1, (i * M + j + 1) * 4 + 3);
            if (i + 1 < n) u(base + 2, ((i + 1) * M + j) * 4 + 0);
        }
        int cnt = 0;
        for (int x = 0; x < p.length; x++) if (f(x) == x) cnt++;
        return cnt;
    }
    public static void main(String[] a) {
        String[] grid = {"\\    /", " \\  /", "  \\/"};
        System.out.println(countRegions(grid)); // 3
    }
}
