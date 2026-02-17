// Count smaller elements to the right via Fenwick tree + coordinate compression.
// Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static int[] tree;
    static void update(int i) { for (; i < tree.length; i += i & -i) tree[i]++; }
    static int query(int i) { int s = 0; for (; i > 0; i -= i & -i) s += tree[i]; return s; }

    static int[] countSmaller(int[] a) {
        int n = a.length;
        int[] sorted = a.clone();
        Arrays.sort(sorted);
        // unique values for rank lookup
        TreeMap<Integer, Integer> rank = new TreeMap<>();
        int r = 0;
        for (int v : sorted) if (!rank.containsKey(v)) rank.put(v, ++r);
        tree = new int[r + 1];
        int[] res = new int[n];
        for (int i = n - 1; i >= 0; i--) {
            int rk = rank.get(a[i]);
            res[i] = query(rk - 1);
            update(rk);
        }
        return res;
    }

    public static void main(String[] args) {
        int[] a = {3, 4, 9, 6, 1};
        System.out.println(Arrays.toString(countSmaller(a)));
    }
}
