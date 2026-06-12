// Maximum spanning tree: Kruskal with edges sorted DESC by weight + Union-Find (path compression, union by rank). O(E log E).
import java.util.Arrays;

public class Solution {
    static int[] parent, rank_;

    static int find(int x) {
        while (parent[x] != x) { parent[x] = parent[parent[x]]; x = parent[x]; }
        return x;
    }

    static boolean unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;
        if (rank_[a] < rank_[b]) { int t = a; a = b; b = t; }
        parent[b] = a;
        if (rank_[a] == rank_[b]) rank_[a]++;
        return true;
    }

    static int maxSpanningTree(int n, int[][] edges) {
        Arrays.sort(edges, (x, y) -> Integer.compare(y[2], x[2]));
        parent = new int[n];
        rank_ = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        int total = 0;
        for (int[] e : edges)
            if (unite(e[0], e[1])) total += e[2];
        return total;
    }

    public static void main(String[] args) {
        int[][] edges = {{0,1,1},{0,2,2},{0,3,3},{1,2,4},{2,3,5}};
        System.out.println("Maximum spanning tree weight: " + maxSpanningTree(4, edges));
    }
}
