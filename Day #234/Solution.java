// Maximum spanning tree: Kruskal with edges sorted by descending weight + union-find.
// Time: O(E log E), Space: O(V).
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

    static boolean unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;
        parent[a] = b;
        return true;
    }

    static int maxSpanningTree(int n, int[][] edges, List<int[]> chosen) {
        Arrays.sort(edges, (x, y) -> y[2] - x[2]);
        parent = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        int total = 0;
        for (int[] e : edges) {
            if (unite(e[0], e[1])) {
                total += e[2];
                chosen.add(new int[]{e[0], e[1]});
            }
        }
        return total;
    }

    public static void main(String[] args) {
        int n = 4;
        int[][] edges = {{0, 1, 1}, {1, 2, 2}, {2, 3, 3}, {0, 3, 4}, {0, 2, 5}};
        List<int[]> chosen = new ArrayList<>();
        int total = maxSpanningTree(n, edges, chosen);
        System.out.println("Max spanning tree weight: " + total); // 11
        StringBuilder sb = new StringBuilder("Edges: ");
        for (int[] e : chosen) sb.append("(").append(e[0]).append("-").append(e[1]).append(") ");
        System.out.println(sb.toString().trim());
    }
}
