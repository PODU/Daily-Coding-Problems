// Day 374: Lowest index i with arr[i]==i in a sorted, distinct-int array.
// f(i)=arr[i]-i is non-decreasing, so binary-search the leftmost i with
// f(i)>=0 and check equality. Time O(log n), Space O(1).
public class Solution {
    static Integer fixedPoint(int[] arr) {
        int lo = 0, hi = arr.length - 1, ans = -1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (arr[mid] >= mid) { ans = mid; hi = mid - 1; }
            else lo = mid + 1;
        }
        return (ans != -1 && arr[ans] == ans) ? ans : null;
    }

    public static void main(String[] args) {
        int[] arr = {-5, -3, 2, 3};
        Integer r = fixedPoint(arr);
        System.out.println(r == null ? "null" : r); // 2
    }
}
