// Largest number: sort by comparator a+b > b+a (concatenation), then join.
// Time: O(n log n * L), Space: O(n * L).
import java.util.*;

public class Solution {
    static String largestNumber(int[] nums) {
        String[] s = new String[nums.length];
        for (int i = 0; i < nums.length; i++) s[i] = Integer.toString(nums[i]);
        Arrays.sort(s, (a, b) -> (b + a).compareTo(a + b));
        if (s[0].equals("0")) return "0";
        StringBuilder sb = new StringBuilder();
        for (String x : s) sb.append(x);
        return sb.toString();
    }

    public static void main(String[] args) {
        int[] nums = {10, 7, 76, 415};
        System.out.println(largestNumber(nums)); // 77641510
    }
}
