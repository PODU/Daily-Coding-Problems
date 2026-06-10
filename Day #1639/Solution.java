// Count inversions using modified merge sort (count cross-pairs during merge).
// Time: O(N log N), Space: O(N).
import java.util.Arrays;

public class Solution {
    static long mergeCount(int[] a, int[] tmp, int lo, int hi) {
        if (hi - lo <= 1) return 0;
        int mid = lo + (hi - lo) / 2;
        long inv = mergeCount(a, tmp, lo, mid) + mergeCount(a, tmp, mid, hi);
        int i = lo, j = mid, k = lo;
        while (i < mid && j < hi) {
            if (a[i] <= a[j]) tmp[k++] = a[i++];
            else { tmp[k++] = a[j++]; inv += (mid - i); }
        }
        while (i < mid) tmp[k++] = a[i++];
        while (j < hi) tmp[k++] = a[j++];
        for (int t = lo; t < hi; t++) a[t] = tmp[t];
        return inv;
    }

    static long countInversions(int[] src) {
        int[] a = Arrays.copyOf(src, src.length);
        int[] tmp = new int[a.length];
        return mergeCount(a, tmp, 0, a.length);
    }

    public static void main(String[] args) {
        System.out.println("[2, 4, 1, 3, 5] has " + countInversions(new int[]{2, 4, 1, 3, 5}) + " inversions");
        System.out.println("[5, 4, 3, 2, 1] has " + countInversions(new int[]{5, 4, 3, 2, 1}) + " inversions");
    }
}
