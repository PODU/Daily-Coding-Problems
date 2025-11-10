// Word circle: model words as directed edges first->last char; Eulerian circuit via Hierholzer. Time O(V+E), Space O(V+E).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        String[] words = {"chair", "height", "racket", "touch", "tunic"};
        Map<Character, List<String[]>> adj = new HashMap<>(); // char -> list of {nextChar, word}
        for (String w : words) {
            char f = w.charAt(0), l = w.charAt(w.length() - 1);
            adj.computeIfAbsent(f, k -> new ArrayList<>()).add(new String[]{String.valueOf(l), w});
        }
        Map<Character, Integer> ptr = new HashMap<>();

        char start = words[0].charAt(0);
        List<String> path = new ArrayList<>();
        Deque<String[]> st = new ArrayDeque<>(); // frame {char, word|null}
        st.push(new String[]{String.valueOf(start), null});
        while (!st.isEmpty()) {
            char v = st.peek()[0].charAt(0);
            int p = ptr.getOrDefault(v, 0);
            List<String[]> lst = adj.getOrDefault(v, Collections.emptyList());
            if (p < lst.size()) {
                ptr.put(v, p + 1);
                String[] edge = lst.get(p);
                st.push(new String[]{edge[0], edge[1]});
            } else {
                String[] frame = st.pop();
                if (frame[1] != null) path.add(frame[1]);
            }
        }
        Collections.reverse(path);

        StringBuilder sb = new StringBuilder();
        for (String w : path) sb.append(w).append(" --> ");
        sb.append(path.get(0));
        System.out.println(sb.toString());
    }
}
