// Day 216: Roman numeral -> decimal.
// Approach: scan left-to-right; if current value < next, subtract, else add. Time O(n), Space O(1).
import java.util.*;

public class Solution {
    static int romanToInt(String s) {
        Map<Character, Integer> v = Map.of('M',1000,'D',500,'C',100,'L',50,'X',10,'V',5,'I',1);
        int total = 0;
        for (int i = 0; i < s.length(); i++) {
            if (i + 1 < s.length() && v.get(s.charAt(i)) < v.get(s.charAt(i + 1)))
                total -= v.get(s.charAt(i));
            else
                total += v.get(s.charAt(i));
        }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(romanToInt("XIV")); // 14
    }
}
