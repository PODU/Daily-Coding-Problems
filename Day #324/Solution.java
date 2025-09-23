// Assign mice to holes minimizing max distance: sort both, pair i-th, answer = max|mice[i]-holes[i]|.
// Time: O(N log N), Space: O(1) extra.
import java.util.*;

public class Solution {
    static int minMaxDistance(int[] mice, int[] holes) {
        Arrays.sort(mice);
        Arrays.sort(holes);
        int ans = 0;
        for (int i = 0; i < mice.length; i++)
            ans = Math.max(ans, Math.abs(mice[i] - holes[i]));
        return ans;
    }

    public static void main(String[] args) {
        int[] mice = {1, 4, 9, 15};
        int[] holes = {10, -5, 0, 16};
        System.out.println(minMaxDistance(mice, holes));
    }
}
