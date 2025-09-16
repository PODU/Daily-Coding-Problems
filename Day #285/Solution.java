// Day 285: Count buildings (east->west) with a sunset (west) view.
// Single backward pass tracking running max. Time O(N), Space O(1).
public class Solution {
    static int sunsetViews(int[] h) {
        int count = 0, maxSoFar = Integer.MIN_VALUE;
        for (int i = h.length - 1; i >= 0; i--) {
            if (h[i] > maxSoFar) { count++; maxSoFar = h[i]; }
        }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(sunsetViews(new int[]{3, 7, 8, 3, 6, 1})); // 3
    }
}
