// Region cutting by slashes: split each cell into 4 triangles, union-find.
// Time: O(N*M * alpha), Space: O(N*M).
public class Solution {
    static int[] par;

    static int find(int x) {
        while (par[x] != x) { par[x] = par[par[x]]; x = par[x]; }
        return x;
    }

    static void uni(int a, int b) { par[find(a)] = find(b); }

    static int regions(String[] g) {
        int n = g.length, m = g[0].length();
        par = new int[n * m * 4];
        for (int i = 0; i < par.length; i++) par[i] = i;
        for (int r = 0; r < n; r++) {
            for (int c = 0; c < m; c++) {
                int base = (r * m + c) * 4;
                char ch = g[r].charAt(c);
                if (ch == '/') { uni(base + 0, base + 3); uni(base + 1, base + 2); }
                else if (ch == '\\') { uni(base + 0, base + 1); uni(base + 2, base + 3); }
                else { uni(base + 0, base + 1); uni(base + 1, base + 2); uni(base + 2, base + 3); }
                if (c + 1 < m) uni(base + 1, ((r * m + c + 1) * 4) + 3);
                if (r + 1 < n) uni(base + 2, (((r + 1) * m + c) * 4) + 0);
            }
        }
        int cnt = 0;
        for (int i = 0; i < par.length; i++) if (find(i) == i) cnt++;
        return cnt;
    }

    public static void main(String[] args) {
        String[] g = {
            "\\    /",
            " \\  / ",
            "  \\/  "
        };
        System.out.println(regions(g));
    }
}
