// Day 1841: Top-k most similar website pairs by Jaccard similarity of their visitor sets.
// Time O(W^2 * U) over W websites; Space O(total pairs).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        String[][] raw = {
            {"a","1"},{"a","3"},{"a","5"},
            {"b","2"},{"b","6"},
            {"c","1"},{"c","2"},{"c","3"},{"c","4"},{"c","5"},
            {"d","4"},{"d","5"},{"d","6"},{"d","7"},
            {"e","1"},{"e","3"},{"e","5"},{"e","6"},
        };
        int k = 1;
        Map<String, Set<Integer>> sites = new TreeMap<>();
        for (String[] p : raw) sites.computeIfAbsent(p[0], x -> new HashSet<>()).add(Integer.parseInt(p[1]));

        List<String> names = new ArrayList<>(sites.keySet());
        List<Object[]> scored = new ArrayList<>();
        for (int i = 0; i < names.size(); i++)
            for (int j = i + 1; j < names.size(); j++) {
                Set<Integer> A = sites.get(names.get(i)), B = sites.get(names.get(j));
                int inter = 0;
                for (int x : A) if (B.contains(x)) inter++;
                int uni = A.size() + B.size() - inter;
                double jac = uni == 0 ? 0 : (double) inter / uni;
                scored.add(new Object[]{jac, names.get(i), names.get(j)});
            }
        scored.sort((a, b) -> Double.compare((double) b[0], (double) a[0]));

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < k; i++) {
            Object[] s = scored.get(i);
            sb.append("('").append(s[1]).append("', '").append(s[2]).append("')");
            if (i + 1 < k) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
