// Day 1283: For each element, count smaller elements to its right.
// Fenwick (BIT) over compressed values, scanning right-to-left. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static int[] tree;

    static void update(int i, int v) {
        for (; i < tree.length; i += i & -i) tree[i] += v;
    }

    static int query(int i) {
        int s = 0;
        for (; i > 0; i -= i & -i) s += tree[i];
        return s;
    }

    static int[] countSmaller(int[] a) {
        int n = a.length;
        int[] sorted = a.clone();
        Arrays.sort(sorted);
        // distinct ranks via binary search on sorted (duplicates ok with lower-bound)
        tree = new int[n + 1];
        int[] res = new int[n];
        for (int i = n - 1; i >= 0; --i) {
            int rank = lowerBound(sorted, a[i]) + 1;
            res[i] = query(rank - 1);
            update(rank, 1);
        }
        return res;
    }

    static int lowerBound(int[] arr, int key) {
        int lo = 0, hi = arr.length;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (arr[mid] < key) lo = mid + 1;
            else hi = mid;
        }
        return lo;
    }

    public static void main(String[] args) {
        System.out.println(Arrays.toString(countSmaller(new int[]{3, 4, 9, 6, 1})));
        // [1, 1, 2, 1, 0]
    }
}
