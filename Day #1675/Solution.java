// Day 1675: Detect a cycle in an undirected graph via Union-Find.
// Union endpoints; a cycle exists if an edge joins already-connected nodes.
// Time O(E*alpha(V)), Space O(V).
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
        parent[b] = a; if (rank_[a] == rank_[b]) rank_[a]++;
        return true;
    }
    static boolean hasCycle(int n, int[][] edges) {
        parent = new int[n]; rank_ = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        for (int[] e : edges) if (!unite(e[0], e[1])) return true;
        return false;
    }

    public static void main(String[] args) {
        System.out.println(hasCycle(4, new int[][]{{0,1},{1,2},{2,3},{3,0}}) ? "True" : "False"); // True
        System.out.println(hasCycle(4, new int[][]{{0,1},{1,2},{2,3}}) ? "True" : "False");       // False
    }
}
