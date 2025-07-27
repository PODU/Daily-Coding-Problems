// Reconstruct Itinerary: backtracking DFS over lexicographically sorted edges.
// First complete itinerary using all edges (tried in lex order) is the answer.
// Time: exponential worst case; Space: O(E).
import java.util.*;

public class Solution {
    static Map<String, List<String>> adj;
    static Map<String, boolean[]> used;
    static int totalEdges;
    static List<String> path;

    static boolean dfs(String node) {
        if (path.size() == totalEdges + 1) return true;
        List<String> dests = adj.get(node);
        if (dests == null) return false;
        boolean[] u = used.get(node);
        for (int i = 0; i < dests.size(); i++) {
            if (u[i]) continue;
            u[i] = true;
            path.add(dests.get(i));
            if (dfs(dests.get(i))) return true;
            path.remove(path.size() - 1);
            u[i] = false;
        }
        return false;
    }

    static List<String> reconstruct(String[][] flights, String start) {
        adj = new HashMap<>();
        used = new HashMap<>();
        for (String[] f : flights) adj.computeIfAbsent(f[0], k -> new ArrayList<>()).add(f[1]);
        for (Map.Entry<String, List<String>> e : adj.entrySet()) {
            Collections.sort(e.getValue());
            used.put(e.getKey(), new boolean[e.getValue().size()]);
        }
        totalEdges = flights.length;
        path = new ArrayList<>();
        path.add(start);
        return dfs(start) ? path : null;
    }

    static String fmt(List<String> v) {
        if (v == null) return "null";
        StringBuilder s = new StringBuilder("[");
        for (int i = 0; i < v.size(); i++) {
            s.append("'").append(v.get(i)).append("'");
            if (i + 1 < v.size()) s.append(", ");
        }
        return s.append("]").toString();
    }

    public static void main(String[] args) {
        System.out.println(fmt(reconstruct(new String[][]{{"SFO","HKO"},{"YYZ","SFO"},{"YUL","YYZ"},{"HKO","ORD"}}, "YUL")));
        System.out.println(fmt(reconstruct(new String[][]{{"SFO","COM"},{"COM","YYZ"}}, "COM")));
        System.out.println(fmt(reconstruct(new String[][]{{"A","B"},{"A","C"},{"B","C"},{"C","A"}}, "A")));
    }
}
