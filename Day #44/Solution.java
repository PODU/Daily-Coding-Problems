// Count Inversions via modified merge sort: count cross-pairs while merging.
// Time O(n log n), Space O(n).
public class Solution {
    static long mergeCount(int[] a, int lo, int hi, int[] tmp) {
        if (hi - lo <= 1) return 0;
        int mid = (lo + hi) / 2;
        long inv = mergeCount(a, lo, mid, tmp) + mergeCount(a, mid, hi, tmp);
        int i = lo, j = mid, k = lo;
        while (i < mid && j < hi) {
            if (a[i] <= a[j]) tmp[k++] = a[i++];
            else { tmp[k++] = a[j++]; inv += mid - i; }
        }
        while (i < mid) tmp[k++] = a[i++];
        while (j < hi) tmp[k++] = a[j++];
        System.arraycopy(tmp, lo, a, lo, hi - lo);
        return inv;
    }

    static long countInversions(int[] src) {
        int[] a = src.clone();
        return mergeCount(a, 0, a.length, new int[a.length]);
    }

    public static void main(String[] args) {
        System.out.println(countInversions(new int[]{2, 4, 1, 3, 5}));
        System.out.println(countInversions(new int[]{5, 4, 3, 2, 1}));
    }
}
