// Direction-rule consistency: decompose each rule into strict x/y "greater-than"
// edges; a contradiction is a cycle in either axis graph (DFS cycle detection).
// Time: O(V+E) per axis, Space: O(V+E).
import java.util.*;

public class Solution {
    static class Checker {
        Map<String, List<String>> gx = new HashMap<>(), gy = new HashMap<>();
        Set<String> nodes = new TreeSet<>();

        void addEdge(Map<String, List<String>> g, String a, String b) {
            g.computeIfAbsent(a, k -> new ArrayList<>()).add(b);
            nodes.add(a);
            nodes.add(b);
        }

        void addRule(String a, String dir, String b) {
            for (char c : dir.toCharArray()) {
                if (c == 'N') addEdge(gy, a, b);
                if (c == 'S') addEdge(gy, b, a);
                if (c == 'E') addEdge(gx, a, b);
                if (c == 'W') addEdge(gx, b, a);
            }
        }

        boolean dfs(Map<String, List<String>> g, String u, Map<String, Integer> state) {
            state.put(u, 1);
            for (String v : g.getOrDefault(u, Collections.emptyList())) {
                int s = state.getOrDefault(v, 0);
                if (s == 1) return true;
                if (s == 0 && dfs(g, v, state)) return true;
            }
            state.put(u, 2);
            return false;
        }

        boolean hasCycle(Map<String, List<String>> g) {
            Map<String, Integer> state = new HashMap<>();
            for (String n : nodes)
                if (state.getOrDefault(n, 0) == 0 && dfs(g, n, state)) return true;
            return false;
        }

        boolean validates() { return !hasCycle(gx) && !hasCycle(gy); }
    }

    public static void main(String[] args) {
        Checker c1 = new Checker();
        c1.addRule("A", "N", "B");
        c1.addRule("B", "NE", "C");
        c1.addRule("C", "N", "A");
        if (!c1.validates())
            System.out.println("does not validate, since A cannot be both north and south of C.");

        Checker c2 = new Checker();
        c2.addRule("A", "NW", "B");
        c2.addRule("A", "N", "B");
        if (c2.validates())
            System.out.println("A NW B / A N B is considered valid.");
    }
}
