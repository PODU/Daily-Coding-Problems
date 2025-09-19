// Minimum spanning tree cost via Kruskal + union-find over undirected weighted graph.
// Time: O(E log E), Space: O(V + E).
import java.util.*;

public class Solution {
    static int[] parent;
    static int find(int x) { return parent[x] == x ? x : (parent[x] = find(parent[x])); }
    static boolean unite(int a, int b) { a = find(a); b = find(b); if (a == b) return false; parent[a] = b; return true; }

    public static void main(String[] args) {
        Map<String, Map<String,Integer>> pipes = new LinkedHashMap<>();
        pipes.put("plant", new LinkedHashMap<>(Map.of("A",1,"B",5,"C",20)));
        pipes.put("A", new LinkedHashMap<>(Map.of("C",15)));
        pipes.put("B", new LinkedHashMap<>(Map.of("C",10)));
        pipes.put("C", new LinkedHashMap<>());

        Map<String,Integer> id = new HashMap<>();
        for (String node : pipes.keySet()) id.putIfAbsent(node, id.size());

        Set<String> seen = new HashSet<>();
        List<int[]> edges = new ArrayList<>(); // {w, u, v}
        for (Map.Entry<String, Map<String,Integer>> kv : pipes.entrySet()) {
            for (Map.Entry<String,Integer> e : kv.getValue().entrySet()) {
                String a = kv.getKey(), b = e.getKey();
                if (a.compareTo(b) > 0) { String t = a; a = b; b = t; }
                String key = a + "|" + b + "|" + e.getValue();
                if (seen.add(key)) edges.add(new int[]{e.getValue(), id.get(a), id.get(b)});
            }
        }
        edges.sort(Comparator.comparingInt(x -> x[0]));

        parent = new int[id.size()];
        for (int i = 0; i < parent.length; i++) parent[i] = i;

        int total = 0;
        for (int[] e : edges)
            if (unite(e[1], e[2])) total += e[0];

        System.out.println(total);
    }
}
