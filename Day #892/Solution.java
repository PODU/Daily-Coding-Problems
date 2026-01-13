// Power set via bitmask iteration over 2^n subsets, then sorted by (size, elements).
// Time: O(n*2^n), Space: O(n*2^n) to hold all subsets.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] s = {1, 2, 3};
        int n = s.length;
        List<List<Integer>> subsets = new ArrayList<>();
        for (int mask = 0; mask < (1 << n); mask++) {
            List<Integer> cur = new ArrayList<>();
            for (int i = 0; i < n; i++)
                if ((mask & (1 << i)) != 0) cur.add(s[i]);
            subsets.add(cur);
        }
        subsets.sort((a, b) -> {
            if (a.size() != b.size()) return a.size() - b.size();
            for (int i = 0; i < a.size(); i++)
                if (!a.get(i).equals(b.get(i))) return a.get(i) - b.get(i);
            return 0;
        });
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < subsets.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append("{");
            for (int j = 0; j < subsets.get(i).size(); j++) {
                if (j > 0) sb.append(", ");
                sb.append(subsets.get(i).get(j));
            }
            sb.append("}");
        }
        sb.append("}");
        System.out.println(sb.toString());
    }
}
