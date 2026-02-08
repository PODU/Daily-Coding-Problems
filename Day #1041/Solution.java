// Trapping rain water via two pointers tracking leftMax/rightMax. Time O(N), Space O(1).
public class Solution {
    static int trap(int[] h) {
        int l = 0, r = h.length - 1, lm = 0, rm = 0, water = 0;
        while (l < r) {
            if (h[l] < h[r]) {
                lm = Math.max(lm, h[l]);
                water += lm - h[l];
                l++;
            } else {
                rm = Math.max(rm, h[r]);
                water += rm - h[r];
                r--;
            }
        }
        return water;
    }

    public static void main(String[] args) {
        System.out.println("[2, 1, 2] -> " + trap(new int[]{2, 1, 2}));
        System.out.println("[3, 0, 1, 3, 0, 5] -> " + trap(new int[]{3, 0, 1, 3, 0, 5}));
    }
}
