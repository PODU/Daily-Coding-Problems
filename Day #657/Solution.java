// Power set via bitmasks; sort subsets by size then numeric order. Time O(n*2^n), Space O(2^n).
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
            out.append("{");
            for (int j = 0; j < subsets.get(i).size(); j++) {
                out.append(subsets.get(i).get(j));
                if (j + 1 < subsets.get(i).size()) out.append(", ");
            }
            out.append("}");
            if (i + 1 < subsets.size()) out.append(", ");
        }
        out.append("}");
        System.out.println(out.toString());
    }
}
