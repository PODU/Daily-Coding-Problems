// Day 865: Minimum cost to connect all houses to the plant = MST cost.
// Approach: Kruskal's algorithm with union-find over all pipe edges.
// Time: O(E log E), Space: O(V + E).
import java.util.*;

public class Solution {
    static int[] parent;
    static int find(int x) { return parent[x] == x ? x : (parent[x] = find(parent[x])); }
    static boolean unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;
        parent[a] = b; return true;
    }

    public static void main(String[] args) {
        Map<String, Integer> id = new HashMap<>();
        int[][] data; // built below
        String[][] raw = {{"plant","A","1"},{"plant","B","5"},{"plant","C","20"},
                          {"A","C","15"},{"B","C","10"}};
        List<int[]> edges = new ArrayList<>();
        for (String[] e : raw) {
            id.putIfAbsent(e[0], id.size());
            id.putIfAbsent(e[1], id.size());
            edges.add(new int[]{Integer.parseInt(e[2]), id.get(e[0]), id.get(e[1])});
        }
        edges.sort((a, b) -> a[0] - b[0]);
        parent = new int[id.size()];
        for (int i = 0; i < parent.length; i++) parent[i] = i;

        int total = 0;
        for (int[] e : edges) if (unite(e[1], e[2])) total += e[0];
        System.out.println(total); // 16
    }
}
