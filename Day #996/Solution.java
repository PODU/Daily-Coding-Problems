// Day 996: Maximum weight spanning tree.
// Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
// O(E log E) time, O(V) space.
import java.util.*;

public class Solution {
    static int[] parent;

    static int find(int x) {
        while (parent[x] != x) { parent[x] = parent[parent[x]]; x = parent[x]; }
        return x;
    }

    static boolean union(int a, int b) {
        int ra = find(a), rb = find(b);
        if (ra == rb) return false;
        parent[ra] = rb;
        return true;
    }

    public static void main(String[] args) {
        int n = 4;
        // {weight, u, v}
        int[][] edges = {{1,0,1},{3,0,2},{2,1,2},{4,1,3},{5,2,3}};
        Arrays.sort(edges, (a, b) -> b[0] - a[0]); // heaviest first
        parent = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        int total = 0;
        for (int[] e : edges)
            if (union(e[1], e[2])) total += e[0];
        System.out.println("Maximum spanning tree weight: " + total); // 12
    }
}
