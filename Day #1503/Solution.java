// Power set via bitmask, sorted by (size, lexicographic) to match example order.
// Time O(n*2^n), Space O(2^n).
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
        for (int k = 0; k < subsets.size(); k++) {
            if (k > 0) out.append(", ");
            out.append("{");
            List<Integer> sub = subsets.get(k);
            for (int i = 0; i < sub.size(); i++) {
                if (i > 0) out.append(", ");
                out.append(sub.get(i));
            }
            out.append("}");
        }
        out.append("}");
        System.out.println(out);
    }
}
