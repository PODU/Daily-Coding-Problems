// Count regions split by '/','\\',' ' via Union-Find: each cell = 4 triangles (T,R,B,L).
// Union within cell per char and across neighbors. Time O(R*C*alpha), Space O(R*C).
import java.util.*;

public class Solution {
    static int[] parent;

    static int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }

    static void uni(int a, int b) {
        parent[find(a)] = find(b);
    }

    static int regions(String[] grid) {
        int R = grid.length, C = grid[0].length();
        parent = new int[4 * R * C];
        for (int i = 0; i < parent.length; i++) parent[i] = i;
        for (int r = 0; r < R; r++) {
            for (int c = 0; c < C; c++) {
                char ch = grid[r].charAt(c);
                int base = 4 * (r * C + c);
                if (ch == ' ') {
                    uni(base, base + 1);
                    uni(base + 1, base + 2);
                    uni(base + 2, base + 3);
                } else if (ch == '/') {
                    uni(base, base + 3);
                    uni(base + 1, base + 2);
                } else {
                    uni(base, base + 1);
                    uni(base + 2, base + 3);
                }
                if (c + 1 < C) uni(base + 1, 4 * (r * C + c + 1) + 3);
                if (r + 1 < R) uni(base + 2, 4 * ((r + 1) * C + c) + 0);
            }
        }
        Set<Integer> roots = new HashSet<>();
        for (int i = 0; i < parent.length; i++) roots.add(find(i));
        return roots.size();
    }

    public static void main(String[] args) {
        String[] grid = {
            "\\    /",
            " \\  / ",
            "  \\/  "
        };
        System.out.println(regions(grid)); // 3
    }
}
