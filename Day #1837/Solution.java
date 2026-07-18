// Day 1837: Stable marriage via Gale-Shapley (men propose). Always yields a stable matching.
// Time O(N^2), Space O(N^2) for preference/ranking tables.
import java.util.*;

public class Solution {
    static Map<String, String> stableMatch(
            Map<String, List<String>> guyPref,
            Map<String, List<String>> galPref) {
        Map<String, Map<String, Integer>> galRank = new HashMap<>();
        for (var e : galPref.entrySet()) {
            Map<String, Integer> r = new HashMap<>();
            List<String> pref = e.getValue();
            for (int i = 0; i < pref.size(); i++) r.put(pref.get(i), i);
            galRank.put(e.getKey(), r);
        }
        Map<String, Integer> next = new HashMap<>();
        Map<String, String> galPartner = new HashMap<>();
        Deque<String> free = new ArrayDeque<>();
        for (String guy : guyPref.keySet()) { free.add(guy); next.put(guy, 0); }

        while (!free.isEmpty()) {
            String guy = free.poll();
            String gal = guyPref.get(guy).get(next.get(guy));
            next.put(guy, next.get(guy) + 1);
            if (!galPartner.containsKey(gal)) {
                galPartner.put(gal, guy);
            } else {
                String cur = galPartner.get(gal);
                if (galRank.get(gal).get(guy) < galRank.get(gal).get(cur)) {
                    galPartner.put(gal, guy);
                    free.add(cur);
                } else {
                    free.add(guy);
                }
            }
        }
        Map<String, String> guyPartner = new TreeMap<>();
        for (var e : galPartner.entrySet()) guyPartner.put(e.getValue(), e.getKey());
        return guyPartner;
    }

    public static void main(String[] args) {
        Map<String, List<String>> guy = new LinkedHashMap<>();
        guy.put("andrew", List.of("caroline", "abigail", "betty"));
        guy.put("bill", List.of("caroline", "betty", "abigail"));
        guy.put("chester", List.of("betty", "caroline", "abigail"));
        Map<String, List<String>> gal = new LinkedHashMap<>();
        gal.put("abigail", List.of("andrew", "bill", "chester"));
        gal.put("betty", List.of("bill", "andrew", "chester"));
        gal.put("caroline", List.of("bill", "chester", "andrew"));
        for (var e : stableMatch(guy, gal).entrySet())
            System.out.println(e.getKey() + " -> " + e.getValue());
    }
}
