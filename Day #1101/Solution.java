// Day 1101: Assign mice to holes minimizing the maximum travel distance.
// Sort both, match in order; answer is max |mice[i]-holes[i]|.
// Time: O(N log N). Space: O(1).
import java.util.*;

public class Solution {
    static int minLastMouse(int[] mice, int[] holes){
        Arrays.sort(mice);
        Arrays.sort(holes);
        int ans = 0;
        for (int i = 0; i < mice.length; i++) ans = Math.max(ans, Math.abs(mice[i]-holes[i]));
        return ans;
    }
    public static void main(String[] args){
        System.out.println(minLastMouse(new int[]{1,4,9,15}, new int[]{10,-5,0,16})); // 6
    }
}
