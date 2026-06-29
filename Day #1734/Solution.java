// Binary search for fixed point (A[i]==i) in sorted distinct array; A[i]-i non-decreasing.
// Time: O(log n), Space: O(1).
public class Solution {
    static String fixedPoint(int[] a) {
        int lo = 0, hi = a.length - 1, ans = -1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (a[mid] == mid) { ans = mid; hi = mid - 1; }
            else if (a[mid] < mid) lo = mid + 1;
            else hi = mid - 1;
        }
        return ans == -1 ? "False" : Integer.toString(ans);
    }

    public static void main(String[] args) {
        System.out.println(fixedPoint(new int[]{-6, 0, 2, 40}));
        System.out.println(fixedPoint(new int[]{1, 5, 7, 8}));
    }
}
