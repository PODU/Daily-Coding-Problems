// Power set via bitmask enumeration, sorted by (size, lexicographic). O(2^n * n).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] nums = {1, 2, 3};
        int n = nums.length;
        List<List<Integer>> subsets = new ArrayList<>();
        for (int mask = 0; mask < (1 << n); mask++) {
            List<Integer> s = new ArrayList<>();
            for (int i = 0; i < n; i++)
                if ((mask & (1 << i)) != 0) s.add(nums[i]);
            subsets.add(s);
        }
        subsets.sort((a, b) -> {
            if (a.size() != b.size()) return a.size() - b.size();
            for (int i = 0; i < a.size(); i++)
                if (!a.get(i).equals(b.get(i))) return a.get(i) - b.get(i);
            return 0;
        });
        List<String> parts = new ArrayList<>();
        for (List<Integer> s : subsets) {
            List<String> e = new ArrayList<>();
            for (int v : s) e.add(Integer.toString(v));
            parts.add("[" + String.join(", ", e) + "]");
        }
        System.out.println("[" + String.join(", ", parts) + "]");
    }
}
