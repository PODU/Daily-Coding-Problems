// Day 1213: Stable marriage via Gale-Shapley (men propose).
// Each free man proposes down his list; women keep their best suitor. Time O(N^2), Space O(N^2).
import java.util.*;

public class Solution {
    static Map<String,String> stableMatch(Map<String,List<String>> guys, Map<String,List<String>> gals) {
        Map<String,Map<String,Integer>> rank = new HashMap<>();
        for (var e : gals.entrySet()) {
            Map<String,Integer> r = new HashMap<>();
            for (int i = 0; i < e.getValue().size(); i++) r.put(e.getValue().get(i), i);
            rank.put(e.getKey(), r);
        }
        Map<String,Integer> next = new HashMap<>();
        Map<String,String> engaged = new HashMap<>();
        Deque<String> free = new ArrayDeque<>();
        for (String m : guys.keySet()) { free.add(m); next.put(m, 0); }
        while (!free.isEmpty()) {
            String m = free.poll();
            String w = guys.get(m).get(next.get(m));
            next.put(m, next.get(m) + 1);
            if (!engaged.containsKey(w)) engaged.put(w, m);
            else {
                String cur = engaged.get(w);
                if (rank.get(w).get(m) < rank.get(w).get(cur)) { engaged.put(w, m); free.add(cur); }
                else free.add(m);
            }
        }
        return engaged;
    }

    public static void main(String[] args) {
        Map<String,List<String>> guys = new LinkedHashMap<>();
        guys.put("andrew", List.of("caroline","abigail","betty"));
        guys.put("bill", List.of("caroline","betty","abigail"));
        guys.put("chester", List.of("betty","caroline","abigail"));
        Map<String,List<String>> gals = new LinkedHashMap<>();
        gals.put("abigail", List.of("andrew","bill","chester"));
        gals.put("betty", List.of("bill","andrew","chester"));
        gals.put("caroline", List.of("bill","chester","andrew"));
        Map<String,String> m = stableMatch(guys, gals);
        TreeMap<String,String> byMan = new TreeMap<>();
        for (var e : m.entrySet()) byMan.put(e.getValue(), e.getKey());
        for (var e : byMan.entrySet()) System.out.println(e.getKey() + " - " + e.getValue());
        // andrew - abigail / bill - caroline / chester - betty
    }
}
