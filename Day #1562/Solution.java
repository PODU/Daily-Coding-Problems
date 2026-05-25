// Sort mice and holes, pair by index, answer = max |mice[i]-holes[i]|. Time O(n log n), Space O(1).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] mice = {1, 4, 9, 15};
        int[] holes = {10, -5, 0, 16};
        Arrays.sort(mice);
        Arrays.sort(holes);
        int ans = 0;
        for (int i = 0; i < mice.length; i++)
            ans = Math.max(ans, Math.abs(mice[i] - holes[i]));
        System.out.println(ans);
    }
}
