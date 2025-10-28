// Streaming vote tally: hash set of seen voters (duplicate -> fraud, vote dropped),
// hash map candidate->count, top 3 computed on demand. Time O(n + k log k), Space O(n+k).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        String[][] stream = {
            {"v1", "A"}, {"v2", "A"}, {"v3", "B"}, {"v4", "C"},
            {"v5", "B"}, {"v6", "B"}, {"v7", "C"}, {"v2", "D"}
        };

        Set<String> seen = new HashSet<>();
        Map<String, Integer> tally = new HashMap<>();

        for (String[] rec : stream) {
            String voter = rec[0], cand = rec[1];
            if (seen.contains(voter)) {
                System.out.println("Fraud detected: voter " + voter + " voted more than once");
                continue; // do not count fraudulent vote
            }
            seen.add(voter);
            tally.merge(cand, 1, Integer::sum);
        }

        List<Map.Entry<String, Integer>> v = new ArrayList<>(tally.entrySet());
        v.sort((a, b) -> {
            if (!a.getValue().equals(b.getValue())) return b.getValue() - a.getValue();
            return a.getKey().compareTo(b.getKey());
        });

        StringBuilder sb = new StringBuilder("Top 3: ");
        int n = Math.min(3, v.size());
        for (int i = 0; i < n; i++) {
            sb.append(v.get(i).getKey()).append("(").append(v.get(i).getValue()).append(")");
            if (i + 1 < n) sb.append(", ");
        }
        System.out.println(sb.toString());
    }
}
