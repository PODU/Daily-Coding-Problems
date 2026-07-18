// Day 1838: Count smaller elements to the right, via a Fenwick tree over compressed values.
// Time O(N log N), Space O(N).
import java.util.*;

public class Solution {
    static int[] tree;

    static void add(int i) { for (; i < tree.length; i += i & -i) tree[i]++; }
    static int sum(int i) { int s = 0; for (; i > 0; i -= i & -i) s += tree[i]; return s; }

    static int[] countSmaller(int[] a) {
        int n = a.length;
        int[] sorted = a.clone();
        Arrays.sort(sorted);
        // distinct values -> rank map via binary search on sorted unique array
        int[] uniq = Arrays.stream(sorted).distinct().toArray();
        tree = new int[uniq.length + 1];
        int[] res = new int[n];
        for (int i = n - 1; i >= 0; i--) {
            int r = lowerBound(uniq, a[i]) + 1; // 1-indexed rank
            res[i] = sum(r - 1);
            add(r);
        }
        return res;
    }

    static int lowerBound(int[] arr, int x) {
        int lo = 0, hi = arr.length;
        while (lo < hi) { int m = (lo + hi) >>> 1; if (arr[m] < x) lo = m + 1; else hi = m; }
        return lo;
    }

    public static void main(String[] args) {
        System.out.println(Arrays.toString(countSmaller(new int[]{3, 4, 9, 6, 1})));
    }
}
