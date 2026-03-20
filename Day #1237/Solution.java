// Count connected components (friend groups) via Union-Find.
// Time O(V+E a(V)), Space O(V).
import java.util.*;

public class Solution {
    static int[] p;
    static int find(int x) { while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; } return x; }
    static void unite(int a, int b) { p[find(a)] = find(b); }

    static int countGroups(Map<Integer, int[]> adj) {
        int n = adj.size();
        p = new int[n];
        for (int i = 0; i < n; i++) p[i] = i;
        for (Map.Entry<Integer, int[]> e : adj.entrySet())
            for (int v : e.getValue())
                unite(e.getKey(), v);
        Set<Integer> roots = new HashSet<>();
        for (int k : adj.keySet()) roots.add(find(k));
        return roots.size();
    }

    public static void main(String[] args) {
        Map<Integer, int[]> adj = new HashMap<>();
        adj.put(0, new int[]{1, 2});
        adj.put(1, new int[]{0, 5});
        adj.put(2, new int[]{0});
        adj.put(3, new int[]{6});
        adj.put(4, new int[]{});
        adj.put(5, new int[]{1});
        adj.put(6, new int[]{3});
        System.out.println(countGroups(adj));
    }
}
