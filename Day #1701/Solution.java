// Stream voting: hashmap vote counts + set of seen voters; duplicate voter = fraud.
// Top 3 by count (ties by candidate id). Time O(n + k log k), Space O(k+v).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Object[][] records = {{1,"A"},{2,"B"},{3,"A"},{4,"C"},{2,"A"},{5,"B"},{6,"A"}};
        Map<String,Integer> counts = new HashMap<>();
        Set<Integer> seen = new HashSet<>();
        for (Object[] r : records) {
            int voter = (Integer) r[0];
            String cand = (String) r[1];
            if (seen.contains(voter)) {
                System.out.println("Fraud detected: voter " + voter + " voted more than once");
                continue;
            }
            seen.add(voter);
            counts.merge(cand, 1, Integer::sum);
        }
        List<Map.Entry<String,Integer>> v = new ArrayList<>(counts.entrySet());
        v.sort((a,b) -> a.getValue().equals(b.getValue()) ? a.getKey().compareTo(b.getKey()) : b.getValue() - a.getValue());
        StringBuilder sb = new StringBuilder("Top 3 candidates: ");
        for (int i = 0; i < 3 && i < v.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append(v.get(i).getKey()).append("(").append(v.get(i).getValue()).append(")");
        }
        System.out.println(sb);
    }
}
