// Fixed point in sorted distinct array via binary search (nums[i]-i non-decreasing).
// Time: O(log n), Space: O(1).
public class Solution {
    static int fixedPoint(int[] nums) {
        int lo = 0, hi = nums.length - 1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (nums[mid] == mid) return mid;
            else if (nums[mid] < mid) lo = mid + 1;
            else hi = mid - 1;
        }
        return -1;
    }

    public static void main(String[] args) {
        int[] a = {-6, 0, 2, 40};
        int[] b = {1, 5, 7, 8};
        int r1 = fixedPoint(a);
        System.out.println(r1 == -1 ? "False" : Integer.toString(r1));
        int r2 = fixedPoint(b);
        System.out.println(r2 == -1 ? "False" : Integer.toString(r2));
    }
}
