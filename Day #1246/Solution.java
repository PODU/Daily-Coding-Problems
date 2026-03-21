// Day 1246: Maximum weight spanning tree.
// Approach: Kruskal's algorithm with edges sorted in DESCENDING weight + union-find.
// Time: O(E log E), Space: O(V + E).
import java.util.*;

public class Solution {
    static int[] parent, rank_;
    static int find(int x) { return parent[x] == x ? x : (parent[x] = find(parent[x])); }
    static boolean unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;
        if (rank_[a] < rank_[b]) { int t = a; a = b; b = t; }
        parent[b] = a;
        if (rank_[a] == rank_[b]) rank_[a]++;
        return true;
    }

    // edges: {weight, u, v}
    static long maxSpanningTree(int n, int[][] edges) {
        parent = new int[n];
        rank_ = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        Arrays.sort(edges, (x, y) -> Integer.compare(y[0], x[0]));
        long total = 0;
        for (int[] e : edges)
            if (unite(e[1], e[2])) total += e[0];
        return total;
    }

    public static void main(String[] args) {
        int n = 4;
        int[][] edges = {{1,0,1},{2,1,2},{3,2,3},{4,0,3},{5,0,2}};
        System.out.println(maxSpanningTree(n, edges)); // 11
    }
}
