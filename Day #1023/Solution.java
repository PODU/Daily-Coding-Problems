// Day 1023: Alien dictionary - order of letters from sorted words.
// Approach: build precedence graph from adjacent words, Kahn's topological sort.
// Time O(total chars + V + E), Space O(V + E).
import java.util.*;

public class Solution {
    static List<Character> alienOrder(String[] words) {
        List<Character> appear = new ArrayList<>();
        Map<Character, TreeSet<Character>> adj = new HashMap<>();
        Map<Character, Integer> indeg = new LinkedHashMap<>();
        for (String w : words)
            for (char c : w.toCharArray())
                if (!indeg.containsKey(c)) {
                    indeg.put(c, 0);
                    appear.add(c);
                    adj.put(c, new TreeSet<>());
                }

        for (int i = 0; i + 1 < words.length; i++) {
            String a = words[i], b = words[i + 1];
            int n = Math.min(a.length(), b.length()), j = 0;
            while (j < n && a.charAt(j) == b.charAt(j)) j++;
            if (j < n) {
                char u = a.charAt(j), v = b.charAt(j);
                if (adj.get(u).add(v)) indeg.put(v, indeg.get(v) + 1);
            }
        }

        Deque<Character> q = new ArrayDeque<>();
        for (char c : appear) if (indeg.get(c) == 0) q.add(c);
        List<Character> res = new ArrayList<>();
        while (!q.isEmpty()) {
            char c = q.poll();
            res.add(c);
            for (char nb : adj.get(c)) {
                indeg.put(nb, indeg.get(nb) - 1);
                if (indeg.get(nb) == 0) q.add(nb);
            }
        }
        return res;
    }

    public static void main(String[] args) {
        String[] words = {"xww", "wxyz", "wxyw", "ywx", "ywz"};
        List<Character> res = alienOrder(words);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("'").append(res.get(i)).append("'");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
