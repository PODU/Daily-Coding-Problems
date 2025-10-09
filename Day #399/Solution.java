// Partition into 3 contiguous equal-sum parts: greedy prefix cut at target, absorbing trailing zeros. O(n) time, O(n) space.
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static List<List<Integer>> partition3(int[] L) {
        long total = 0;
        for (int x : L) total += x;
        if (total % 3 != 0) return null;
        long target = total / 3;
        List<List<Integer>> res = new ArrayList<>();
        List<Integer> cur = new ArrayList<>();
        long running = 0;
        for (int i = 0; i < L.length; i++) {
            cur.add(L[i]);
            running += L[i];
            // close part when sum hits target and next element is non-zero (zeros stay attached)
            if (res.size() < 2 && running == target && (i + 1 == L.length || L[i + 1] != 0)) {
                res.add(cur);
                cur = new ArrayList<>();
                running = 0;
            }
        }
        res.add(cur);
        if (res.size() != 3) return null;
        for (List<Integer> p : res) {
            long s = 0;
            for (int x : p) s += x;
            if (s != target) return null;
        }
        return res;
    }

    public static void main(String[] args) {
        int[] L = {3, 5, 8, 0, 8};
        List<List<Integer>> parts = partition3(L);
        System.out.println(parts == null ? "None" : parts.toString());
    }
}
