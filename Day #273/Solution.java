// Day 273: Fixed point (arr[i]==i) in sorted distinct array via binary search.
// Time O(log N), Space O(1). Returns index or -1 (False).
public class Solution {
    static int fixedPoint(int[] a) {
        int lo = 0, hi = a.length - 1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (a[mid] == mid) return mid;
            else if (a[mid] < mid) lo = mid + 1;
            else hi = mid - 1;
        }
        return -1;
    }

    static void show(int[] a) {
        int r = fixedPoint(a);
        System.out.println(r == -1 ? "False" : Integer.toString(r));
    }

    public static void main(String[] args) {
        show(new int[]{-6, 0, 2, 40}); // 2
        show(new int[]{1, 5, 7, 8});   // False
    }
}
