// Stream voting: track seen voters + candidate counts; report top-3 (count desc, id asc) or FRAUD on repeat.
// Time: O(n * c log c) over stream, Space: O(voters + candidates).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Object[][] stream = {
            {1,"A"},{2,"B"},{3,"A"},{4,"C"},{5,"B"},{1,"A"},{6,"A"}
        };

        Set<Integer> seen = new HashSet<>();
        Map<String,Integer> counts = new HashMap<>();

        for (Object[] rec : stream) {
            int voter = (Integer) rec[0];
            String cand = (String) rec[1];
            if (seen.contains(voter)) {
                System.out.println("Fraud: voter " + voter + " voted more than once");
                continue;
            }
            seen.add(voter);
            counts.merge(cand, 1, Integer::sum);

            List<Map.Entry<String,Integer>> v = new ArrayList<>(counts.entrySet());
            v.sort((a, b) -> {
                if (!a.getValue().equals(b.getValue())) return b.getValue() - a.getValue();
                return a.getKey().compareTo(b.getKey());
            });

            StringBuilder out = new StringBuilder("Top 3: [");
            int limit = Math.min(v.size(), 3);
            for (int i = 0; i < limit; i++) {
                if (i > 0) out.append(", ");
                out.append("'").append(v.get(i).getKey()).append("'");
            }
            out.append("]");
            System.out.println(out);
        }
    }
}
