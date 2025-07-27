// Power set via bitmask 0..2^n-1, then order subsets by (size, element order).
// O(2^n * n) time, O(2^n * n) space.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] s = {1, 2, 3};
        int n = s.length;
        List<List<Integer>> subsets = new ArrayList<>();
        for (int mask = 0; mask < (1 << n); mask++) {
            List<Integer> sub = new ArrayList<>();
            for (int i = 0; i < n; i++)
                if ((mask & (1 << i)) != 0) sub.add(s[i]);
            subsets.add(sub);
        }
        subsets.sort((a, b) -> {
            if (a.size() != b.size()) return a.size() - b.size();
            for (int i = 0; i < a.size(); i++)
                if (!a.get(i).equals(b.get(i))) return a.get(i) - b.get(i);
            return 0;
        });
        StringBuilder out = new StringBuilder("{");
        for (int i = 0; i < subsets.size(); i++) {
            if (i > 0) out.append(", ");
            out.append("{");
            List<Integer> sub = subsets.get(i);
            for (int j = 0; j < sub.size(); j++) {
                if (j > 0) out.append(", ");
                out.append(sub.get(j));
            }
            out.append("}");
        }
        out.append("}");
        System.out.println(out.toString());
    }
}
