// Trapping Rain Water: two-pointer sweep tracking left/right running maxima.
// Time: O(N), Space: O(1).
public class Solution {
    static long trap(int[] h) {
        int l = 0, r = h.length - 1;
        int leftMax = 0, rightMax = 0;
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
        System.out.println(trap(new int[]{2, 1, 2}));
        System.out.println(trap(new int[]{3, 0, 1, 3, 0, 5}));
    }
}
