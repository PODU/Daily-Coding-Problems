// Top-k similar website pairs by Jaccard similarity of user sets.
// Build per-site user sets, compute Jaccard for all pairs, sort desc (ties alpha), take k.
// Time: O(S^2 * U), Space: O(S*U).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Object[][] visits = {
            {"a",1},{"a",3},{"a",5},{"b",2},{"b",6},
            {"c",1},{"c",2},{"c",3},{"c",4},{"c",5},
            {"d",4},{"d",5},{"d",6},{"d",7},
            {"e",1},{"e",3},{"e",5},{"e",6}
        };
        int k = 1;

        TreeMap<String, Set<Integer>> sites = new TreeMap<>();
        for (Object[] v : visits) {
            sites.computeIfAbsent((String) v[0], x -> new HashSet<>()).add((Integer) v[1]);
        }
        List<String> names = new ArrayList<>(sites.keySet());

        List<double[]> simIdx = new ArrayList<>();
        List<String[]> pairs = new ArrayList<>();
        for (int i = 0; i < names.size(); i++) {
            for (int j = i + 1; j < names.size(); j++) {
                Set<Integer> A = sites.get(names.get(i));
                Set<Integer> B = sites.get(names.get(j));
                Set<Integer> inter = new HashSet<>(A);
                inter.retainAll(B);
                int uni = A.size() + B.size() - inter.size();
                double sim = uni == 0 ? 0.0 : (double) inter.size() / uni;
                simIdx.add(new double[]{sim, pairs.size()});
                pairs.add(new String[]{names.get(i), names.get(j)});
            }
        }
        simIdx.sort((x, y) -> {
            if (x[0] != y[0]) return Double.compare(y[0], x[0]);
            String[] px = pairs.get((int) x[1]), py = pairs.get((int) y[1]);
            if (!px[0].equals(py[0])) return px[0].compareTo(py[0]);
            return px[1].compareTo(py[1]);
        });

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < k && i < simIdx.size(); i++) {
            if (i > 0) sb.append(", ");
            String[] p = pairs.get((int) simIdx.get(i)[1]);
            sb.append("('").append(p[0]).append("', '").append(p[1]).append("')");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
