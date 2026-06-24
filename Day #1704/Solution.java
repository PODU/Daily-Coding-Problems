// Lazy bartender = min set cover (NP-hard). Greedy: repeatedly pick drink covering
// most uncovered customers. Time O(D^2*C), Space O(D*C).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Map<Integer, int[]> preferences = new LinkedHashMap<>();
        preferences.put(0, new int[]{0,1,3,6});
        preferences.put(1, new int[]{1,4,7});
        preferences.put(2, new int[]{2,4,7,5});
        preferences.put(3, new int[]{3,2,5});
        preferences.put(4, new int[]{5,8});

        Map<Integer, Set<Integer>> drinkToCust = new HashMap<>();
        Set<Integer> uncovered = new HashSet<>();
        for (Map.Entry<Integer, int[]> e : preferences.entrySet()) {
            uncovered.add(e.getKey());
            for (int d : e.getValue())
                drinkToCust.computeIfAbsent(d, k -> new HashSet<>()).add(e.getKey());
        }
        int learned = 0;
        while (!uncovered.isEmpty()) {
            int bestDrink = -1, bestCount = 0;
            for (Map.Entry<Integer, Set<Integer>> e : drinkToCust.entrySet()) {
                int cnt = 0;
                for (int c : e.getValue()) if (uncovered.contains(c)) cnt++;
                if (cnt > bestCount) { bestCount = cnt; bestDrink = e.getKey(); }
            }
            if (bestDrink == -1) break;
            uncovered.removeAll(drinkToCust.get(bestDrink));
            learned++;
        }
        System.out.println(learned);
    }
}
