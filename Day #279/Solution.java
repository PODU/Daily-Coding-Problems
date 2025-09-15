// Day 279: Count friend groups = connected components via Union-Find.
// Time O(V + E * alpha(V)), Space O(V).
import java.util.*;

public class Solution {
    static int[] p, rnk;

    static int find(int x) {
        while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; }
        return x;
    }

    static void unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return;
        if (rnk[a] < rnk[b]) { int t = a; a = b; b = t; }
        p[b] = a;
        if (rnk[a] == rnk[b]) rnk[a]++;
    }

    static int countGroups(Map<Integer, List<Integer>> adj) {
        int n = adj.size();
        p = new int[n];
        rnk = new int[n];
        for (int i = 0; i < n; i++) p[i] = i;
        for (var e : adj.entrySet())
            for (int v : e.getValue()) unite(e.getKey(), v);
        Set<Integer> roots = new HashSet<>();
        for (int u : adj.keySet()) roots.add(find(u));
        return roots.size();
    }

    public static void main(String[] args) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        adj.put(0, Arrays.asList(1, 2));
        adj.put(1, Arrays.asList(0, 5));
        adj.put(2, Arrays.asList(0));
        adj.put(3, Arrays.asList(6));
        adj.put(4, Arrays.asList());
        adj.put(5, Arrays.asList(1));
        adj.put(6, Arrays.asList(3));
        System.out.println(countGroups(adj)); // 3
    }
}
