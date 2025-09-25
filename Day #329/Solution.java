// Gale-Shapley stable marriage, men propose; free man proposes down his list, women keep better partner.
// Time O(n^2), Space O(n^2).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        String[] men = {"andrew", "bill", "chester"};
        String[] women = {"abigail", "betty", "caroline"};
        Map<String, String[]> guyPref = new HashMap<>();
        guyPref.put("andrew", new String[]{"caroline", "abigail", "betty"});
        guyPref.put("bill", new String[]{"caroline", "betty", "abigail"});
        guyPref.put("chester", new String[]{"betty", "caroline", "abigail"});
        Map<String, String[]> galPref = new HashMap<>();
        galPref.put("abigail", new String[]{"andrew", "bill", "chester"});
        galPref.put("betty", new String[]{"bill", "andrew", "chester"});
        galPref.put("caroline", new String[]{"bill", "chester", "andrew"});

        Map<String, Map<String, Integer>> wrank = new HashMap<>();
        for (String w : women) {
            Map<String, Integer> r = new HashMap<>();
            String[] list = galPref.get(w);
            for (int i = 0; i < list.length; i++) r.put(list[i], i);
            wrank.put(w, r);
        }

        Map<String, Integer> next = new HashMap<>();
        for (String m : men) next.put(m, 0);
        Map<String, String> partnerOf = new HashMap<>(); // woman -> man
        Deque<String> freeMen = new ArrayDeque<>(Arrays.asList(men));

        while (!freeMen.isEmpty()) {
            String m = freeMen.pollFirst();
            String w = guyPref.get(m)[next.get(m)];
            next.put(m, next.get(m) + 1);
            if (!partnerOf.containsKey(w)) {
                partnerOf.put(w, m);
            } else {
                String cur = partnerOf.get(w);
                if (wrank.get(w).get(m) < wrank.get(w).get(cur)) {
                    partnerOf.put(w, m);
                    freeMen.addLast(cur);
                } else {
                    freeMen.addLast(m);
                }
            }
        }

        Map<String, String> manToWoman = new HashMap<>();
        for (Map.Entry<String, String> e : partnerOf.entrySet())
            manToWoman.put(e.getValue(), e.getKey());
        for (String m : men) // already sorted
            System.out.println(m + " - " + manToWoman.get(m));
    }
}
