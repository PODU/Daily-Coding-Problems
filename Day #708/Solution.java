// Day 708: Fixed point (a[i]==i) in a sorted distinct array via binary search.
// Since values are distinct integers, a[i]-i is monotonic. Time O(log n).
public class Solution {
    static int fixedPoint(int[] a) {
        int lo = 0, hi = a.length - 1;
        while (lo <= hi) {
            int mid = (lo + hi) / 2;
            if (a[mid] == mid) return mid;
            else if (a[mid] < mid) lo = mid + 1;
            else hi = mid - 1;
        }
        return -1;
    }

    static void report(int[] a) {
        int r = fixedPoint(a);
        System.out.println(r >= 0 ? Integer.toString(r) : "False");
    }

    public static void main(String[] args) {
        report(new int[]{-6, 0, 2, 40});
        report(new int[]{1, 5, 7, 8});
    }
}
