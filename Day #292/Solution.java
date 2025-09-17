// Bipartite check (2-coloring) via BFS over enemy graph; split into two teams or False.
// Time O(V+E), Space O(V).
import java.util.*;

public class Solution {
    static boolean twoColor(Map<Integer,List<Integer>> g, int[] color, int n) {
        for (int s = 0; s < n; s++) {
            if (color[s] != -1) continue;
            Queue<Integer> q = new LinkedList<>(); q.add(s); color[s] = 0;
            while (!q.isEmpty()) {
                int u = q.poll();
                for (int v : g.get(u)) {
                    if (color[v] == -1) { color[v] = color[u] ^ 1; q.add(v); }
                    else if (color[v] == color[u]) return false;
                }
            }
        }
        return true;
    }

    static void solve(Map<Integer,List<Integer>> g, int n) {
        int[] color = new int[n]; Arrays.fill(color, -1);
        if (!twoColor(g, color, n)) { System.out.println("False"); return; }
        List<Integer> a = new ArrayList<>(), b = new ArrayList<>();
        for (int i = 0; i < n; i++) (color[i] == 0 ? a : b).add(i);
        System.out.println(fmt(a) + " and " + fmt(b));
    }

    static String fmt(List<Integer> s) {
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < s.size(); i++) { sb.append(s.get(i)); if (i+1<s.size()) sb.append(", "); }
        return sb.append("}").toString();
    }

    public static void main(String[] args) {
        Map<Integer,List<Integer>> g1 = new HashMap<>();
        g1.put(0,List.of(3)); g1.put(1,List.of(2)); g1.put(2,List.of(1,4));
        g1.put(3,List.of(0,4,5)); g1.put(4,List.of(2,3)); g1.put(5,List.of(3));
        Map<Integer,List<Integer>> g2 = new HashMap<>();
        g2.put(0,List.of(3)); g2.put(1,List.of(2)); g2.put(2,List.of(1,3,4));
        g2.put(3,List.of(0,2,4,5)); g2.put(4,List.of(2,3)); g2.put(5,List.of(3));
        solve(g1, 6);
        solve(g2, 6);
    }
}
