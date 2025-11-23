// Day 646: Count friend groups = connected components in an undirected graph.
// Approach: Union-Find (DSU) with path compression + union by rank.
// Time: O(V + E * alpha(V)), Space: O(V).
import java.util.*;

public class Solution {
    static int[] parent, rank_;
    static int find(int x) { return parent[x] == x ? x : (parent[x] = find(parent[x])); }
    static void unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return;
        if (rank_[a] < rank_[b]) { int t = a; a = b; b = t; }
        parent[b] = a;
        if (rank_[a] == rank_[b]) rank_[a]++;
    }

    static int countGroups(Map<Integer, List<Integer>> adj) {
        int n = adj.size();
        parent = new int[n];
        rank_ = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        for (var e : adj.entrySet())
            for (int v : e.getValue()) unite(e.getKey(), v);
        Set<Integer> roots = new HashSet<>();
        for (int k : adj.keySet()) roots.add(find(k));
        return roots.size();
    }

    public static void main(String[] args) {
        Map<Integer, List<Integer>> adj = new LinkedHashMap<>();
        adj.put(0, List.of(1, 2));
        adj.put(1, List.of(0, 5));
        adj.put(2, List.of(0));
        adj.put(3, List.of(6));
        adj.put(4, List.of());
        adj.put(5, List.of(1));
        adj.put(6, List.of(3));
        System.out.println(countGroups(adj)); // 3
    }
}
