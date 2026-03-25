// Day 1266: Arrange numbers to form the largest integer.
// Sort by custom comparator a+b vs b+a (descending). O(n log n) comparisons.
import java.util.*;

public class Solution {
    static String largestNumber(int[] nums) {
        String[] s = new String[nums.length];
        for (int i = 0; i < nums.length; i++) s[i] = Integer.toString(nums[i]);
        Arrays.sort(s, (a, b) -> (b + a).compareTo(a + b));
        if (s.length == 0 || s[0].equals("0")) return "0";
        StringBuilder sb = new StringBuilder();
        for (String t : s) sb.append(t);
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(largestNumber(new int[]{10, 7, 76, 415}));
    }
}
