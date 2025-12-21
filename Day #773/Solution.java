// Day 773: Count inversions via modified merge sort. O(n log n) time, O(n) space.
public class Solution {
    static long mergeCount(int[] a, int l, int r, int[] tmp) {
        if (r - l <= 1) return 0;
        int m = (l + r) / 2;
        long cnt = mergeCount(a, l, m, tmp) + mergeCount(a, m, r, tmp);
        int i = l, j = m, k = l;
        while (i < m && j < r) {
            if (a[i] <= a[j]) tmp[k++] = a[i++];
            else { tmp[k++] = a[j++]; cnt += m - i; }
        }
        while (i < m) tmp[k++] = a[i++];
        while (j < r) tmp[k++] = a[j++];
        System.arraycopy(tmp, l, a, l, r - l);
        return cnt;
    }

    static long countInversions(int[] a) {
        int[] copy = a.clone();
        return mergeCount(copy, 0, copy.length, new int[copy.length]);
    }

    public static void main(String[] args) {
        System.out.println(countInversions(new int[]{2, 4, 1, 3, 5})); // 3
        System.out.println(countInversions(new int[]{5, 4, 3, 2, 1})); // 10
    }
}
