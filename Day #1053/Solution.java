// Day 1053: Directional consistency. Decompose each rule into strict x/y
// inequalities, build a directed "greater-than" graph per axis, and detect a
// cycle (contradiction) via DFS. Time O(V+E), Space O(V+E).

import java.util.*;

public class Solution {
    static void add(Map<String, Set<String>> g, String u, String v) {
        g.computeIfAbsent(u, k -> new TreeSet<>()).add(v);
        g.computeIfAbsent(v, k -> new TreeSet<>());
    }

    static boolean dfs(Map<String, Set<String>> g, String u, Map<String, Integer> state) {
        state.put(u, 0);
        for (String w : g.getOrDefault(u, Collections.emptySet())) {
            if (state.getOrDefault(w, -1) == 0) return true;
            if (!state.containsKey(w) && dfs(g, w, state)) return true;
        }
        state.put(u, 1);
        return false;
    }

    static boolean hasCycle(Map<String, Set<String>> g) {
        Map<String, Integer> state = new HashMap<>();
        for (String n : g.keySet())
            if (!state.containsKey(n) && dfs(g, n, state)) return true;
        return false;
    }

    static boolean validate(List<String> rules) {
        Map<String, Set<String>> gx = new TreeMap<>(), gy = new TreeMap<>();
        for (String rule : rules) {
            String[] p = rule.split("\\s+");
            String a = p[0], d = p[1], b = p[2];
            for (char ch : d.toCharArray()) {
                if (ch == 'N') add(gy, a, b);
                else if (ch == 'S') add(gy, b, a);
                else if (ch == 'E') add(gx, a, b);
                else if (ch == 'W') add(gx, b, a);
            }
        }
        return !(hasCycle(gx) || hasCycle(gy));
    }

    public static void main(String[] args) {
        List<String> ex1 = List.of("A N B", "B NE C", "C N A");
        List<String> ex2 = List.of("A NW B", "A N B");
        System.out.println(!validate(ex1)
            ? "does not validate, since A cannot be both north and south of C."
            : "is considered valid.");
        System.out.println(validate(ex2) ? "is considered valid." : "does not validate.");
    }
}
