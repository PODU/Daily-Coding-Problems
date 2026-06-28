// Day 1729: Count buildings with a sunset (west) view.
// Single right-to-left pass tracking max height seen to the west; a building is
// visible iff strictly taller than all to its west. Time: O(n). Space: O(1).
public class Solution {
    static int countSunsetViews(int[] heights) {
        int count = 0, maxWest = 0;
        // Scan from the west end (rightmost index) toward the east.
        for (int i = heights.length - 1; i >= 0; i--) {
            if (heights[i] > maxWest) {
                count++;
                maxWest = heights[i];
            }
        }
        return count;
    }

    public static void main(String[] args) {
        int[] heights = {3, 7, 8, 3, 6, 1}; // east -> west
        System.out.println(countSunsetViews(heights)); // 1, 6, 8 visible => 3
    }
}
