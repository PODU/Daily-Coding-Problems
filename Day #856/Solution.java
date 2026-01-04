// Day 856: Top k similar website pairs.
// Approach: Jaccard similarity (|A∩B| / |A∪B|) over user sets, take top k pairs.
// Time: O(W^2 * U), Space: O(W * U).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Object[][] visits = {
            {"a",1},{"a",3},{"a",5},
            {"b",2},{"b",6},
            {"c",1},{"c",2},{"c",3},{"c",4},{"c",5},
            {"d",4},{"d",5},{"d",6},{"d",7},
            {"e",1},{"e",3},{"e",5},{"e",6}
        };
        int k = 1;

        TreeMap<String, Set<Integer>> sites = new TreeMap<>();
        for (Object[] v : visits)
            sites.computeIfAbsent((String) v[0], x -> new HashSet<>()).add((Integer) v[1]);

        List<String> names = new ArrayList<>(sites.keySet());
        List<double[]> idx = new ArrayList<>();
        List<String[]> pairs = new ArrayList<>();
        for (int i = 0; i < names.size(); i++)
            for (int j = i + 1; j < names.size(); j++) {
                Set<Integer> A = sites.get(names.get(i)), B = sites.get(names.get(j));
                int inter = 0;
                for (int u : A) if (B.contains(u)) inter++;
                int uni = A.size() + B.size() - inter;
                double sim = uni == 0 ? 0 : (double) inter / uni;
                idx.add(new double[]{sim, pairs.size()});
                pairs.add(new String[]{names.get(i), names.get(j)});
            }
        idx.sort((x, y) -> Double.compare(y[0], x[0]));

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < k && i < idx.size(); i++) {
            if (i > 0) sb.append(", ");
            String[] p = pairs.get((int) idx.get(i)[1]);
            sb.append("('").append(p[0]).append("', '").append(p[1]).append("')");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
