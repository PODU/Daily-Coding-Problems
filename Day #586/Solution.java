// Group users into a set per site; for every site pair compute Jaccard = |A&B|/|A|B|.
// Sort by similarity DESC, tie-break by pair lexicographically; take top k. Time O(P^2 * U).
import java.util.*;

public class Solution {
    static class Cand {
        double sim; String x, y;
        Cand(double s, String a, String b) { sim = s; x = a; y = b; }
    }

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
        for (Object[] v : visits) {
            sites.computeIfAbsent((String) v[0], z -> new HashSet<>()).add((Integer) v[1]);
        }

        List<String> names = new ArrayList<>(sites.keySet());
        List<Cand> cands = new ArrayList<>();
        for (int i = 0; i < names.size(); i++) {
            for (int j = i + 1; j < names.size(); j++) {
                Set<Integer> A = sites.get(names.get(i));
                Set<Integer> B = sites.get(names.get(j));
                int inter = 0;
                for (int u : A) if (B.contains(u)) inter++;
                int uni = A.size() + B.size() - inter;
                double sim = uni == 0 ? 0.0 : (double) inter / uni;
                cands.add(new Cand(sim, names.get(i), names.get(j)));
            }
        }
        cands.sort((a, b) -> {
            if (a.sim != b.sim) return Double.compare(b.sim, a.sim);
            if (!a.x.equals(b.x)) return a.x.compareTo(b.x);
            return a.y.compareTo(b.y);
        });

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < k && i < cands.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append("('").append(cands.get(i).x).append("', '").append(cands.get(i).y).append("')");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
