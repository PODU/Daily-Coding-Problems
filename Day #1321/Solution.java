// Day 1321: Roman numeral -> decimal.
// Approach: single left-to-right pass; subtract if a smaller value precedes a larger one. O(n) time, O(1) space.
import java.util.*;

public class Solution {
    static int romanToInt(String s) {
        Map<Character,Integer> v = new HashMap<>();
        v.put('M',1000); v.put('D',500); v.put('C',100); v.put('L',50);
        v.put('X',10); v.put('V',5); v.put('I',1);
        int total = 0;
        for (int i = 0; i < s.length(); i++) {
            int cur = v.get(s.charAt(i));
            if (i + 1 < s.length() && cur < v.get(s.charAt(i+1))) total -= cur;
            else total += cur;
        }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(romanToInt("XIV")); // 14
    }
}
