// Stream voting: HashMap candidate->count, HashSet of voters to detect fraud; top-3 via sort.
// Time: O(records) processing + O(C log C) reporting. Space: O(C + V).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        String[][] stream = {
            {"v1","A"},{"v2","B"},{"v3","A"},{"v4","C"},
            {"v5","B"},{"v6","A"},{"v7","C"},{"v1","B"}
        };

        Map<String,Integer> counts = new HashMap<>();
        Set<String> seen = new HashSet<>();

        for (String[] rec : stream) {
            String voter = rec[0], cand = rec[1];
            if (seen.contains(voter)) {
                System.out.println("Fraud detected: voter " + voter);
                continue;
            }
            seen.add(voter);
            counts.merge(cand, 1, Integer::sum);
        }

        List<Map.Entry<String,Integer>> v = new ArrayList<>(counts.entrySet());
        v.sort((a, b) -> {
            if (!a.getValue().equals(b.getValue())) return b.getValue() - a.getValue();
            return a.getKey().compareTo(b.getKey());
        });

        StringBuilder sb = new StringBuilder("Top 3 candidates: ");
        for (int i = 0; i < v.size() && i < 3; i++) {
            if (i > 0) sb.append(", ");
            sb.append(v.get(i).getKey());
        }
        System.out.println(sb.toString());
    }
}
