// Day 725: Assign mice to holes minimizing the maximum distance any mouse moves.
// Approach: Sort both arrays, pair in order, answer = max |mice[i]-holes[i]|.
// Time: O(n log n), Space: O(1).
import java.util.*;

public class Solution {
    static int minLastMouse(int[] mice, int[] holes) {
        Arrays.sort(mice);
        Arrays.sort(holes);
        int ans = 0;
        for (int i = 0; i < mice.length; i++)
            ans = Math.max(ans, Math.abs(mice[i] - holes[i]));
        return ans;
    }

    public static void main(String[] args) {
        int[] mice = {1, 4, 9, 15}, holes = {10, -5, 0, 16};
        System.out.println(minLastMouse(mice, holes)); // 6
    }
}
