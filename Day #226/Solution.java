// Alien Dictionary: build precedence graph from adjacent words, topological sort (Kahn's BFS).
// Time: O(total characters), Space: O(unique letters + edges).
import java.util.*;

public class Solution {
    static List<Character> alienOrder(String[] words) {
        Map<Character, Set<Character>> adj = new TreeMap<>();
        Map<Character, Integer> indeg = new TreeMap<>();
        for (String w : words)
            for (char c : w.toCharArray()) { adj.putIfAbsent(c, new TreeSet<>()); indeg.putIfAbsent(c, 0); }
        for (int i = 0; i + 1 < words.length; i++) {
            String a = words[i], b = words[i + 1];
            int n = Math.min(a.length(), b.length());
            for (int j = 0; j < n; j++) {
                if (a.charAt(j) != b.charAt(j)) {
                    if (adj.get(a.charAt(j)).add(b.charAt(j)))
                        indeg.put(b.charAt(j), indeg.get(b.charAt(j)) + 1);
                    break;
                }
            }
        }
        Queue<Character> q = new LinkedList<>();
        for (var e : indeg.entrySet()) if (e.getValue() == 0) q.add(e.getKey());
        List<Character> order = new ArrayList<>();
        while (!q.isEmpty()) {
            char c = q.poll();
            order.add(c);
            for (char nx : adj.get(c)) {
                indeg.put(nx, indeg.get(nx) - 1);
                if (indeg.get(nx) == 0) q.add(nx);
            }
        }
        return order;
    }

    public static void main(String[] args) {
        String[] words = {"xww", "wxyz", "wxyw", "ywx", "ywz"};
        List<Character> o = alienOrder(words);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < o.size(); i++) {
            sb.append("'").append(o.get(i)).append("'");
            if (i + 1 < o.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
