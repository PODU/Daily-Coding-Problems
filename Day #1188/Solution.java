// Count inversions via merge sort: count cross-pairs while merging. Time O(N log N), Space O(N).
public class Solution {
    static long mergeCount(int[] a, int lo, int hi, int[] tmp) {
        if (hi - lo <= 1) return 0;
        int mid = (lo + hi) / 2;
        long inv = mergeCount(a, lo, mid, tmp) + mergeCount(a, mid, hi, tmp);
        int i = lo, j = mid, k = lo;
        while (i < mid && j < hi) {
            if (a[i] <= a[j]) tmp[k++] = a[i++];
            else { tmp[k++] = a[j++]; inv += (mid - i); }
        }
        while (i < mid) tmp[k++] = a[i++];
        while (j < hi) tmp[k++] = a[j++];
        for (int x = lo; x < hi; x++) a[x] = tmp[x];
        return inv;
    }

    public static void main(String[] args) {
        int[] arr = {2, 4, 1, 3, 5};
        int[] tmp = new int[arr.length];
        System.out.println(mergeCount(arr, 0, arr.length, tmp));
    }
}
