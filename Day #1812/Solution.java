// Split students into two teams so no enemies share a team = graph 2-coloring (bipartite check).
// BFS coloring over each component. Time: O(V+E). Space: O(V).
import java.util.*;

public class Solution {
    static boolean divideTeams(Map<Integer, List<Integer>> g, TreeSet<Integer> teamA, TreeSet<Integer> teamB) {
        Map<Integer, Integer> color = new HashMap<>();
        for (int s : g.keySet()) {
            if (color.containsKey(s)) continue;
            Queue<Integer> q = new LinkedList<>();
            q.add(s); color.put(s, 0);
            while (!q.isEmpty()) {
                int u = q.poll();
                for (int v : g.get(u)) {
                    if (!color.containsKey(v)) { color.put(v, color.get(u) ^ 1); q.add(v); }
                    else if (color.get(v).equals(color.get(u))) return false;
                }
            }
        }
        for (Map.Entry<Integer, Integer> e : color.entrySet())
            (e.getValue() == 0 ? teamA : teamB).add(e.getKey());
        return true;
    }

    public static void main(String[] args) {
        Map<Integer, List<Integer>> g = new TreeMap<>();
        g.put(0, Arrays.asList(3));
        g.put(1, Arrays.asList(2));
        g.put(2, Arrays.asList(1, 4));
        g.put(3, Arrays.asList(0, 4, 5));
        g.put(4, Arrays.asList(2, 3));
        g.put(5, Arrays.asList(3));
        TreeSet<Integer> A = new TreeSet<>(), B = new TreeSet<>();
        if (divideTeams(g, A, B)) System.out.println(fmt(A) + " and " + fmt(B));
        else System.out.println("False");
        // {0, 1, 4, 5} and {2, 3}
    }

    static String fmt(TreeSet<Integer> s) {
        StringBuilder sb = new StringBuilder("{");
        boolean first = true;
        for (int x : s) { if (!first) sb.append(", "); sb.append(x); first = false; }
        return sb.append("}").toString();
    }
}
