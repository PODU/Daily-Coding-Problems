// Count regions carved by slashes via 4-triangle split + Union-Find.
// Time O(n*m a(n*m)), Space O(n*m).
public class Solution {
    static int[] p;
    static int count;

    static int find(int x) {
        while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; }
        return x;
    }

    static void unite(int a, int b) {
        int ra = find(a), rb = find(b);
        if (ra != rb) { p[ra] = rb; count--; }
    }

    static int regions(String[] grid) {
        int rows = grid.length, cols = rows > 0 ? grid[0].length() : 0;
        p = new int[4 * rows * cols];
        for (int i = 0; i < p.length; i++) p[i] = i;
        count = p.length;
        for (int r = 0; r < rows; r++)
            for (int c = 0; c < cols; c++) {
                char ch = grid[r].charAt(c);
                int t = 4 * (r * cols + c), ri = t + 1, b = t + 2, le = t + 3;
                if (ch == '/') { unite(t, le); unite(ri, b); }
                else if (ch == '\\') { unite(t, ri); unite(le, b); }
                else { unite(t, ri); unite(ri, b); unite(b, le); }
                if (c + 1 < cols) unite(ri, 4 * (r * cols + c + 1) + 3);
                if (r + 1 < rows) unite(b, 4 * ((r + 1) * cols + c));
            }
        return count;
    }

    public static void main(String[] args) {
        String[] grid = {"\\    /", " \\  / ", "  \\/  "};
        System.out.println(regions(grid));
    }
}
