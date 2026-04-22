// Single pass from the west end (array right), tracking the tallest seen so far;
// a building has a view iff it is taller than everything to its west.
// Time O(n), Space O(1).
public class Solution {
    static int countSunsetViews(int[] h) {
        int count = 0, maxW = Integer.MIN_VALUE;
        for (int i = h.length - 1; i >= 0; i--) {
            if (h[i] > maxW) { count++; maxW = h[i]; }
        }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(countSunsetViews(new int[]{3, 7, 8, 3, 6, 1})); // 3
    }
}
