// Roman numeral -> decimal. Single left-to-right pass; subtract when a smaller
// value precedes a larger one. Time O(n), Space O(1).
import java.util.*;

public class Solution {
    public static int romanToInt(String s) {
        Map<Character, Integer> val = new HashMap<>();
        val.put('M',1000); val.put('D',500); val.put('C',100);
        val.put('L',50); val.put('X',10); val.put('V',5); val.put('I',1);
        int total = 0;
        for (int i = 0; i < s.length(); i++) {
            int v = val.get(s.charAt(i));
            if (i + 1 < s.length() && val.get(s.charAt(i+1)) > v) total -= v;
            else total += v;
        }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(romanToInt("XIV")); // 14
    }
}
