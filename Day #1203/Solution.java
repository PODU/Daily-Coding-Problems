// Day 1203: Alien dictionary - derive letter order from sorted words.
// Build precedence graph from adjacent word diffs, Kahn topological sort. Time O(total chars), Space O(1) alphabet.
import java.util.*;

public class Solution {
    static List<Character> alienOrder(String[] words) {
        Map<Character, TreeSet<Character>> adj = new LinkedHashMap<>();
        Map<Character, Integer> indeg = new LinkedHashMap<>();
        for (String w : words) for (char c : w.toCharArray()) {
            adj.putIfAbsent(c, new TreeSet<>());
            indeg.putIfAbsent(c, 0);
        }
        for (int i = 0; i + 1 < words.length; i++) {
            String a = words[i], b = words[i + 1];
            int n = Math.min(a.length(), b.length());
            for (int j = 0; j < n; j++) if (a.charAt(j) != b.charAt(j)) {
                if (adj.get(a.charAt(j)).add(b.charAt(j))) indeg.merge(b.charAt(j), 1, Integer::sum);
                break;
            }
        }
        Queue<Character> q = new LinkedList<>();
        for (Map.Entry<Character, Integer> e : indeg.entrySet()) if (e.getValue() == 0) q.add(e.getKey());
        List<Character> order = new ArrayList<>();
        while (!q.isEmpty()) {
            char c = q.poll();
            order.add(c);
            for (char nx : adj.get(c)) if (indeg.merge(nx, -1, Integer::sum) == 0) q.add(nx);
        }
        return order;
    }

    public static void main(String[] args) {
        String[] words = {"xww", "wxyz", "wxyw", "ywx", "ywz"};
        List<Character> o = alienOrder(words);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < o.size(); i++) { if (i > 0) sb.append(", "); sb.append("'").append(o.get(i)).append("'"); }
        sb.append("]");
        System.out.println(sb); // ['x', 'z', 'w', 'y']
    }
}
