// Day 593: Count buildings with a view of the setting sun (west).
// Array is east->west (index 0 = east). A building sees the sunset iff it is
// taller than every building further west (higher index). Single right-to-left pass.
// Time: O(n), Space: O(1).
public class Solution {
    static int countSunsetViews(int[] h) {
        int count = 0, maxSeen = Integer.MIN_VALUE;
        for (int i = h.length - 1; i >= 0; i--) {
            if (h[i] > maxSeen) { count++; maxSeen = h[i]; }
        }
        return count;
    }

    public static void main(String[] args) {
        int[] h = {3, 7, 8, 3, 6, 1};
        System.out.println(countSunsetViews(h)); // 3
    }
}
