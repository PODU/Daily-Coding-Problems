// Bipartite 2-coloring via BFS over all components (sorted iteration for determinism).
// Time: O(V+E), Space: O(V+E).
import java.util.*;

public class Solution {
    static List<List<Integer>> solve(TreeMap<Integer, List<Integer>> students) {
        Map<Integer, Integer> color = new HashMap<>();
        for (int start : students.keySet()) {
            if (color.containsKey(start)) continue;
            Deque<Integer> q = new ArrayDeque<>();
            q.add(start);
            color.put(start, 0);
            while (!q.isEmpty()) {
                int u = q.poll();
                List<Integer> nbrs = new ArrayList<>(students.getOrDefault(u, new ArrayList<>()));
                Collections.sort(nbrs);
                for (int v : nbrs) {
                    if (!color.containsKey(v)) {
                        color.put(v, color.get(u) ^ 1);
                        q.add(v);
                    } else if (color.get(v).equals(color.get(u))) {
                        return null;
                    }
                }
            }
        }
        List<Integer> t0 = new ArrayList<>(), t1 = new ArrayList<>();
        for (Map.Entry<Integer, Integer> e : color.entrySet()) {
            (e.getValue() == 0 ? t0 : t1).add(e.getKey());
        }
        Collections.sort(t0);
        Collections.sort(t1);
        return Arrays.asList(t0, t1);
    }

    static String group(List<Integer> g) {
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < g.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append(g.get(i));
        }
        return sb.append("}").toString();
    }

    public static void main(String[] args) {
        TreeMap<Integer, List<Integer>> s1 = new TreeMap<>();
        s1.put(0, Arrays.asList(3));
        s1.put(1, Arrays.asList(2));
        s1.put(2, Arrays.asList(1, 4));
        s1.put(3, Arrays.asList(0, 4, 5));
        s1.put(4, Arrays.asList(2, 3));
        s1.put(5, Arrays.asList(3));
        List<List<Integer>> r1 = solve(s1);
        System.out.println(r1 == null ? "False" : group(r1.get(0)) + " and " + group(r1.get(1)));

        TreeMap<Integer, List<Integer>> s2 = new TreeMap<>();
        s2.put(0, Arrays.asList(3));
        s2.put(1, Arrays.asList(2));
        s2.put(2, Arrays.asList(1, 3, 4));
        s2.put(3, Arrays.asList(0, 2, 4, 5));
        s2.put(4, Arrays.asList(2, 3));
        s2.put(5, Arrays.asList(3));
        List<List<Integer>> r2 = solve(s2);
        System.out.println(r2 == null ? "False" : group(r2.get(0)) + " and " + group(r2.get(1)));
    }
}
