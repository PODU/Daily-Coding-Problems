// Alien dictionary: build edges from first differing chars of adjacent words,
// then Kahn's topological sort. Time O(total chars), Space O(letters + edges).
import java.util.*;

public class Solution {
    static List<Character> alienOrder(String[] words) {
        Map<Character, Set<Character>> adj = new TreeMap<>();
        Map<Character, Integer> indeg = new TreeMap<>();
        for (String w : words)
            for (char c : w.toCharArray()) {
                adj.putIfAbsent(c, new TreeSet<>());
                indeg.putIfAbsent(c, 0);
            }
        for (int i = 0; i + 1 < words.length; i++) {
            String a = words[i], b = words[i + 1];
            int n = Math.min(a.length(), b.length()), j = 0;
            for (; j < n; j++) {
                if (a.charAt(j) != b.charAt(j)) {
                    if (adj.get(a.charAt(j)).add(b.charAt(j)))
                        indeg.merge(b.charAt(j), 1, Integer::sum);
                    break;
                }
            }
            if (j == n && a.length() > b.length()) return new ArrayList<>();
        }
        PriorityQueue<Character> pq = new PriorityQueue<>();
        for (var e : indeg.entrySet()) if (e.getValue() == 0) pq.add(e.getKey());
        List<Character> order = new ArrayList<>();
        while (!pq.isEmpty()) {
            char c = pq.poll();
            order.add(c);
            for (char nx : adj.get(c))
                if (indeg.merge(nx, -1, Integer::sum) == 0) pq.add(nx);
        }
        if (order.size() != indeg.size()) return new ArrayList<>();
        return order;
    }

    public static void main(String[] args) {
        String[] words = {"xww", "wxyz", "wxyw", "ywx", "ywz"};
        List<Character> order = alienOrder(words);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < order.size(); i++) {
            sb.append("'").append(order.get(i)).append("'");
            if (i + 1 < order.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
