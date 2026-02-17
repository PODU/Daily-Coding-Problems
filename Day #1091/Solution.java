// Count connected components via Union-Find with path compression. Time ~O(N+E*alpha), Space O(N).
import java.util.*;

public class Solution {
    static int[] parent;
    static int find(int x) { return parent[x] == x ? x : (parent[x] = find(parent[x])); }
    static void unite(int a, int b) { parent[find(a)] = find(b); }

    static int countGroups(Map<Integer, List<Integer>> adj) {
        int n = adj.size();
        parent = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        for (Map.Entry<Integer, List<Integer>> e : adj.entrySet())
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
        System.out.println(countGroups(adj));
    }
}
