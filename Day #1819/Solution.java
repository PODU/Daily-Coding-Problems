// Longest weighted path (diameter) in a tree via two DFS passes.
// DFS from any node finds one endpoint; DFS from it finds the other + path. Time: O(V+E). Space: O(V).
import java.util.*;

public class Solution {
    static Map<String, List<String[]>> g = new HashMap<>();
    static String bestNode; static long bestDist;
    static Map<String, String> parent;

    static void add(String a, String b, int w) {
        g.computeIfAbsent(a, k -> new ArrayList<>()).add(new String[]{b, "" + w});
        g.computeIfAbsent(b, k -> new ArrayList<>()).add(new String[]{a, "" + w});
    }

    static void dfs(String u, long d, Set<String> vis) {
        vis.add(u);
        if (d > bestDist) { bestDist = d; bestNode = u; }
        for (String[] e : g.getOrDefault(u, Collections.emptyList())) {
            if (!vis.contains(e[0])) { parent.put(e[0], u); dfs(e[0], d + Integer.parseInt(e[1]), vis); }
        }
    }

    static String farthest(String src) {
        parent = new HashMap<>();
        bestNode = src; bestDist = 0;
        dfs(src, 0, new HashSet<>());
        return bestNode;
    }

    public static void main(String[] args) {
        add("a","b",3); add("a","c",5); add("a","d",8);
        add("d","e",2); add("d","f",4); add("e","g",1); add("e","h",1);

        String u = farthest("a");       // one endpoint
        String v = farthest(u);         // other endpoint, parent map now rooted at u
        long len = bestDist;

        List<String> path = new ArrayList<>();
        for (String cur = v; ; cur = parent.get(cur)) { path.add(cur); if (cur.equals(u)) break; }

        System.out.println(String.join(" -> ", path) + ", with a length of " + len);
        // c -> a -> d -> f, with a length of 17
    }
}
