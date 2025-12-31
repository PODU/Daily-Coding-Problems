// Day 830: Largest number formed by concatenating the given numbers.
// Sort strings by comparator (a+b) vs (b+a) descending. O(N log N) compares of O(L) strings.
import java.util.*;

public class Solution {
    static String largestNumber(long[] nums) {
        String[] strs = new String[nums.length];
        for (int i = 0; i < nums.length; i++) strs[i] = Long.toString(nums[i]);
        Arrays.sort(strs, (a, b) -> (b + a).compareTo(a + b));
        StringBuilder sb = new StringBuilder();
        for (String s : strs) sb.append(s);
        String result = sb.toString();
        if (!result.isEmpty() && result.charAt(0) == '0') return "0";
        return result;
    }

    public static void main(String[] args) {
        System.out.println(largestNumber(new long[]{10, 7, 76, 415}));  // 77641510
    }
}
