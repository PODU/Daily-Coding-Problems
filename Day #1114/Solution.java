// Day 1114 - Voting stream: top 3 candidates + fraud detection
// Approach: hash-map vote counts + set of seen voters (O(1) dup detection);
// top-3 via sort. Time: O(R + M log M), Space: O(V+M).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Object[][] stream = {
            {1,"A"},{2,"B"},{3,"A"},{4,"C"},{5,"B"},
            {6,"A"},{2,"C"},{7,"D"},{8,"A"}
        };
        Map<String,Integer> counts = new HashMap<>();
        Set<Integer> seen = new HashSet<>();
        for (Object[] rec : stream) {
            int voter = (Integer) rec[0];
            String cand = (String) rec[1];
            if (seen.contains(voter)) {
                System.out.println("Fraud detected: voter " + voter + " voted more than once");
                continue;
            }
            seen.add(voter);
            counts.merge(cand, 1, Integer::sum);
        }
        List<Map.Entry<String,Integer>> items = new ArrayList<>(counts.entrySet());
        items.sort((a, b) -> a.getValue().equals(b.getValue())
                ? a.getKey().compareTo(b.getKey())
                : b.getValue() - a.getValue());
        StringBuilder sb = new StringBuilder("Top 3 candidates: [");
        for (int i = 0; i < 3 && i < items.size(); i++) {
            sb.append("('").append(items.get(i).getKey()).append("', ")
              .append(items.get(i).getValue()).append(")");
            if (i + 1 < 3 && i + 1 < items.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
