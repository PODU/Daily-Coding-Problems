// Day 1052: Graph bipartiteness / 2-coloring via BFS over all components.
// Time O(V+E), Space O(V). Returns two teams or False if not bipartite.

import java.util.*;

public class Solution {
    static String divideTeams(Map<Integer, List<Integer>> students) {
        Map<Integer, Integer> color = new HashMap<>();
        for (int start : students.keySet()) {
            if (color.containsKey(start)) continue;
            color.put(start, 0);
            Deque<Integer> q = new ArrayDeque<>();
            q.add(start);
            while (!q.isEmpty()) {
                int u = q.poll();
                for (int v : students.get(u)) {
                    if (!color.containsKey(v)) { color.put(v, color.get(u) ^ 1); q.add(v); }
                    else if (color.get(v).equals(color.get(u))) return "False";
                }
            }
        }
        List<Integer> a = new ArrayList<>(), b = new ArrayList<>();
        for (int n : students.keySet()) (color.get(n) == 0 ? a : b).add(n);
        Collections.sort(a); Collections.sort(b);
        return setStr(a) + " and " + setStr(b);
    }

    static String setStr(List<Integer> v) {
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < v.size(); i++) { if (i > 0) sb.append(", "); sb.append(v.get(i)); }
        return sb.append("}").toString();
    }

    public static void main(String[] args) {
        Map<Integer, List<Integer>> s1 = new TreeMap<>();
        s1.put(0, List.of(3)); s1.put(1, List.of(2)); s1.put(2, List.of(1, 4));
        s1.put(3, List.of(0, 4, 5)); s1.put(4, List.of(2, 3)); s1.put(5, List.of(3));
        Map<Integer, List<Integer>> s2 = new TreeMap<>();
        s2.put(0, List.of(3)); s2.put(1, List.of(2)); s2.put(2, List.of(1, 3, 4));
        s2.put(3, List.of(0, 2, 4, 5)); s2.put(4, List.of(2, 3)); s2.put(5, List.of(3));
        System.out.println(divideTeams(s1));
        System.out.println(divideTeams(s2));
    }
}
