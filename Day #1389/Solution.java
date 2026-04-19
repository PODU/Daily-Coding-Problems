// Top-k website pairs by Jaccard similarity over user sets; sort desc, tie-break alpha.
// O(W^2 * U) to compare pairs. Output formatted as Python tuple list.
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

        TreeMap<String, Set<Integer>> users = new TreeMap<>();
        for (Object[] v : visits)
            users.computeIfAbsent((String) v[0], x -> new HashSet<>()).add((Integer) v[1]);

        List<String> sites = new ArrayList<>(users.keySet());
        List<String[]> pairs = new ArrayList<>();
        List<Double> sims = new ArrayList<>();
        for (int i = 0; i < sites.size(); i++)
            for (int j = i + 1; j < sites.size(); j++) {
                Set<Integer> A = users.get(sites.get(i)), B = users.get(sites.get(j));
                int inter = 0;
                for (int u : A) if (B.contains(u)) inter++;
                int uni = A.size() + B.size() - inter;
                pairs.add(new String[]{sites.get(i), sites.get(j)});
                sims.add(uni == 0 ? 0.0 : (double) inter / uni);
            }

        Integer[] idx = new Integer[pairs.size()];
        for (int i = 0; i < idx.length; i++) idx[i] = i;
        Arrays.sort(idx, (x, y) -> {
            int c = Double.compare(sims.get(y), sims.get(x));
            if (c != 0) return c;
            c = pairs.get(x)[0].compareTo(pairs.get(y)[0]);
            if (c != 0) return c;
            return pairs.get(x)[1].compareTo(pairs.get(y)[1]);
        });

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < k && i < idx.length; i++) {
            if (i > 0) sb.append(", ");
            String[] p = pairs.get(idx[i]);
            sb.append("('").append(p[0]).append("', '").append(p[1]).append("')");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
