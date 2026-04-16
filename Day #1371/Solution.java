// Assign mice to holes minimizing max distance: sort both, pair in order.
// Time O(n log n), Space O(1) extra.
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
        System.out.println(minLastMouse(new int[]{1, 4, 9, 15}, new int[]{10, -5, 0, 16}));
    }
}
