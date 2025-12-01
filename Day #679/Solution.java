// Count inversions via modified merge sort. Time O(N log N), Space O(N).
public class Solution {
    static long mergeCount(int[] a, int l, int r) {
        if (r - l <= 1) return 0;
        int m = (l + r) / 2;
        long inv = mergeCount(a, l, m) + mergeCount(a, m, r);
        int[] tmp = new int[r - l];
        int i = l, j = m, k = 0;
        while (i < m && j < r) {
            if (a[i] <= a[j]) tmp[k++] = a[i++];
            else { tmp[k++] = a[j++]; inv += (m - i); }
        }
        while (i < m) tmp[k++] = a[i++];
        while (j < r) tmp[k++] = a[j++];
        for (int t = 0; t < tmp.length; t++) a[l + t] = tmp[t];
        return inv;
    }

    static long countInversions(int[] a) {
        return mergeCount(a.clone(), 0, a.length);
    }

    public static void main(String[] args) {
        System.out.println(countInversions(new int[]{2, 4, 1, 3, 5}));
        System.out.println(countInversions(new int[]{5, 4, 3, 2, 1}));
    }
}
