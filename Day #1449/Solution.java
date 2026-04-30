// Day 1449: Trapping Rain Water. Two-pointer sweep tracking running left/right
// maxima. Time O(n), Space O(1).
public class Solution {
    static long trap(int[] h) {
        int l = 0, r = h.length - 1, leftMax = 0, rightMax = 0;
        long water = 0;
        while (l < r) {
            if (h[l] < h[r]) {
                leftMax = Math.max(leftMax, h[l]);
                water += leftMax - h[l];
                l++;
            } else {
                rightMax = Math.max(rightMax, h[r]);
                water += rightMax - h[r];
                r--;
            }
        }
        return water;
    }

    public static void main(String[] args) {
        System.out.println(trap(new int[]{2, 1, 2}));        // 1
        System.out.println(trap(new int[]{3, 0, 1, 3, 0, 5})); // 8
    }
}
