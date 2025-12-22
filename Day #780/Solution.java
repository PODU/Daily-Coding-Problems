// Day 780: Topological sort of courses (prereqs map). DFS post-order with
// cycle detection; returns null if a cycle exists. O(V + E).
import java.util.*;

public class Solution {
    static boolean dfs(String c, Map<String, List<String>> g,
                       Map<String, Integer> state, List<String> order) {
        state.put(c, 1);
        for (String pre : g.getOrDefault(c, Collections.emptyList())) {
            int s = state.getOrDefault(pre, 0);
            if (s == 1) return false;
            if (s == 0 && !dfs(pre, g, state, order)) return false;
        }
        state.put(c, 2);
        order.add(c);
        return true;
    }

    static List<String> courseOrder(Map<String, List<String>> g) {
        Map<String, Integer> state = new HashMap<>();
        List<String> order = new ArrayList<>();
        for (String c : new TreeSet<>(g.keySet()))
            if (state.getOrDefault(c, 0) == 0 && !dfs(c, g, state, order)) return null;
        return order;
    }

    public static void main(String[] args) {
        Map<String, List<String>> g = new TreeMap<>();
        g.put("CSC300", Arrays.asList("CSC100", "CSC200"));
        g.put("CSC200", Arrays.asList("CSC100"));
        g.put("CSC100", Collections.emptyList());
        List<String> r = courseOrder(g);
        if (r == null) System.out.println("null");
        else {
            StringBuilder sb = new StringBuilder("[");
            for (int i = 0; i < r.size(); i++) {
                sb.append("'").append(r.get(i)).append("'");
                if (i + 1 < r.size()) sb.append(", ");
            }
            System.out.println(sb.append("]")); // ['CSC100', 'CSC200', 'CSC300']
        }
    }
}
