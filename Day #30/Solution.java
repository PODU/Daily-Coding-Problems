// Trapping rain water with two pointers.
// Time: O(n), Space: O(1).
public class Solution {
    static int trap(int[] h) {
        int l = 0, r = h.length - 1;
        int leftMax = 0, rightMax = 0, water = 0;
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
        int[] heights = {3, 0, 1, 3, 0, 5};
        System.out.println(trap(heights));
    }
}
