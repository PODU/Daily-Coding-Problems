// Stable Marriage (Gale-Shapley, men proposing). Each free man proposes down his list.
// Time: O(N^2), Space: O(N^2) for preference rank tables.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Map<String, List<String>> guy = new LinkedHashMap<>();
        guy.put("andrew",  Arrays.asList("caroline","abigail","betty"));
        guy.put("bill",    Arrays.asList("caroline","betty","abigail"));
        guy.put("chester", Arrays.asList("betty","caroline","abigail"));

        Map<String, List<String>> gal = new LinkedHashMap<>();
        gal.put("abigail",  Arrays.asList("andrew","bill","chester"));
        gal.put("betty",    Arrays.asList("bill","andrew","chester"));
        gal.put("caroline", Arrays.asList("bill","chester","andrew"));

        List<String> men = Arrays.asList("andrew","bill","chester");

        Map<String, Map<String,Integer>> rank = new HashMap<>();
        for (Map.Entry<String,List<String>> e : gal.entrySet()) {
            Map<String,Integer> r = new HashMap<>();
            List<String> pref = e.getValue();
            for (int i = 0; i < pref.size(); i++) r.put(pref.get(i), i);
            rank.put(e.getKey(), r);
        }

        Map<String,Integer> next = new HashMap<>();
        for (String m : men) next.put(m, 0);
        Map<String,String> husband = new HashMap<>();
        Deque<String> free = new ArrayDeque<>(men);

        while (!free.isEmpty()) {
            String m = free.pollFirst();
            String w = guy.get(m).get(next.get(m));
            next.put(m, next.get(m) + 1);
            if (!husband.containsKey(w)) {
                husband.put(w, m);
            } else {
                String cur = husband.get(w);
                if (rank.get(w).get(m) < rank.get(w).get(cur)) {
                    husband.put(w, m); free.addLast(cur);
                } else free.addLast(m);
            }
        }

        Map<String,String> wife = new HashMap<>();
        for (Map.Entry<String,String> h : husband.entrySet()) wife.put(h.getValue(), h.getKey());

        StringBuilder sb = new StringBuilder("{");
        boolean first = true;
        for (String m : men) {
            if (!first) sb.append(", ");
            first = false;
            sb.append("'").append(m).append("': '").append(wife.get(m)).append("'");
        }
        sb.append("}");
        System.out.println(sb);
    }
}
