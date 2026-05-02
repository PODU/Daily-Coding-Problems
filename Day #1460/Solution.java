// Fixed point (arr[i]==i) in sorted distinct array via binary search.
// Time O(log n), Space O(1).
public class Solution {
    // Returns index i where arr[i]==i, or -1 if none.
    static int fixedPoint(int[] arr) {
        int lo = 0, hi = arr.length - 1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (arr[mid] == mid) return mid;
            else if (arr[mid] < mid) lo = mid + 1;
            else hi = mid - 1;
        }
        return -1;
    }

    static void run(int[] arr) {
        int r = fixedPoint(arr);
        System.out.println(r == -1 ? "False" : Integer.toString(r));
    }

    public static void main(String[] args) {
        run(new int[]{-6, 0, 2, 40});
        run(new int[]{1, 5, 7, 8});
    }
}
