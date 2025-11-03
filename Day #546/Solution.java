// Count smaller elements to the right via Fenwick/BIT + coordinate compression.
// Time O(n log n), Space O(n).
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
        // unique values
        int m = 0;
        for (int i = 0; i < n; i++) {
            if (i == 0 || sorted[i] != sorted[i - 1]) sorted[m++] = sorted[i];
        }
        int[] uniq = Arrays.copyOf(sorted, m);
        tree = new int[m + 1];
        int[] res = new int[n];
        for (int i = n - 1; i >= 0; i--) {
            int rank = lowerBound(uniq, a[i]) + 1;
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
        int[] a = {3, 4, 9, 6, 1};
        int[] res = countSmaller(a);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.length; i++) {
            sb.append(res[i]);
            if (i + 1 < res.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
