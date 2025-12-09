// Day 721: Maximum-weight spanning tree.
// Approach: Kruskal with edges sorted by DECREASING weight + union-find.
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

    static long maxSpanningTree(int n, int[][] edges) {
        parent = new int[n]; rank_ = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        Arrays.sort(edges, (x, y) -> Integer.compare(y[2], x[2]));
        long total = 0; int used = 0;
        for (int[] e : edges)
            if (unite(e[0], e[1])) { total += e[2]; used++; }
        return used == n - 1 ? total : -1;
    }

    public static void main(String[] args) {
        int n = 4;
        int[][] edges = {{0,1,1},{0,2,2},{0,3,3},{1,2,4},{2,3,5}};
        System.out.println("Maximum spanning tree weight: " + maxSpanningTree(n, edges));
    }
}
