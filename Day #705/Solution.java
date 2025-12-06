// Day 705: Trapping rain water.
// Approach: two pointers tracking running left/right maxima; the smaller side is
// bounded so we can resolve it. Time O(N), Space O(1).
public class Solution {
    static long trap(int[] h) {
        int l = 0, r = h.length - 1, lmax = 0, rmax = 0;
        long water = 0;
        while (l < r) {
            if (h[l] < h[r]) {
                lmax = Math.max(lmax, h[l]); water += lmax - h[l]; l++;
            } else {
                rmax = Math.max(rmax, h[r]); water += rmax - h[r]; r--;
            }
        }
        return water;
    }

    public static void main(String[] args) {
        System.out.println(trap(new int[]{2, 1, 2}));          // 1
        System.out.println(trap(new int[]{3, 0, 1, 3, 0, 5})); // 8
    }
}
