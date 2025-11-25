// Alien dictionary: build edges from first differing char of adjacent words,
// then Kahn's BFS topological sort. Time O(C + V + E), Space O(V + E).
import java.util.*;

public class Solution {
    static List<Character> alienOrder(String[] words) {
        Map<Character, Set<Character>> graph = new HashMap<>();
        Map<Character, Integer> indeg = new HashMap<>();
        for (String w : words)
            for (char c : w.toCharArray()) {
                graph.putIfAbsent(c, new TreeSet<>());
                indeg.putIfAbsent(c, 0);
            }
        for (int i = 0; i + 1 < words.length; i++) {
            String a = words[i], b = words[i + 1];
            int n = Math.min(a.length(), b.length());
            for (int j = 0; j < n; j++) {
                char x = a.charAt(j), y = b.charAt(j);
                if (x != y) {
                    if (graph.get(x).add(y))
                        indeg.put(y, indeg.get(y) + 1);
                    break;
                }
            }
        }
        TreeSet<Character> queue = new TreeSet<>();
        for (Map.Entry<Character, Integer> e : indeg.entrySet())
            if (e.getValue() == 0) queue.add(e.getKey());
        List<Character> order = new ArrayList<>();
        while (!queue.isEmpty()) {
            char c = queue.pollFirst();
            order.add(c);
            for (char nxt : graph.get(c)) {
                indeg.put(nxt, indeg.get(nxt) - 1);
                if (indeg.get(nxt) == 0) queue.add(nxt);
            }
        }
        return order;
    }

    public static void main(String[] args) {
        String[] words = {"xww", "wxyz", "wxyw", "ywx", "ywz"};
        List<Character> order = alienOrder(words);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < order.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append("'").append(order.get(i)).append("'");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
