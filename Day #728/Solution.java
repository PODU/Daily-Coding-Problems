// Day 728: Split students into two teams so no enemies share a team.
// Approach: BFS 2-coloring (bipartite check). Returns two teams or reports failure.
// Time: O(V + E), Space: O(V).
import java.util.*;

public class Solution {
    static int[] twoColor(Map<Integer, List<Integer>> g) {
        int n = g.size();
        int[] color = new int[n];
        Arrays.fill(color, -1);
        for (int s : new TreeSet<>(g.keySet())) {
            if (color[s] != -1) continue;
            color[s] = 0;
            Deque<Integer> q = new ArrayDeque<>();
            q.add(s);
            while (!q.isEmpty()) {
                int u = q.poll();
                for (int v : g.get(u)) {
                    if (color[v] == -1) { color[v] = color[u] ^ 1; q.add(v); }
                    else if (color[v] == color[u]) return null;
                }
            }
        }
        return color;
    }

    static String setStr(List<Integer> v) {
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < v.size(); i++) sb.append(v.get(i)).append(i + 1 < v.size() ? ", " : "");
        return sb.append("}").toString();
    }

    static void solve(Map<Integer, List<Integer>> g) {
        int[] color = twoColor(g);
        if (color == null) { System.out.println("False"); return; }
        List<Integer> a = new ArrayList<>(), b = new ArrayList<>();
        for (int k : new TreeSet<>(g.keySet())) (color[k] == 0 ? a : b).add(k);
        System.out.println(setStr(a) + " and " + setStr(b));
    }

    static Map<Integer, List<Integer>> graph(int[][] adj) {
        Map<Integer, List<Integer>> g = new TreeMap<>();
        for (int i = 0; i < adj.length; i++) {
            List<Integer> l = new ArrayList<>();
            for (int x : adj[i]) l.add(x);
            g.put(i, l);
        }
        return g;
    }

    public static void main(String[] args) {
        solve(graph(new int[][]{{3},{2},{1,4},{0,4,5},{2,3},{3}}));
        solve(graph(new int[][]{{3},{2},{1,3,4},{0,2,4,5},{2,3},{3}}));
    }
}
