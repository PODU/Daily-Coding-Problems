// Roman to decimal: add each value, subtract when a smaller numeral precedes a larger. Time O(n), Space O(1).
import java.util.*;

public class Solution {
    static int romanToInt(String s) {
        Map<Character,Integer> v = new HashMap<>();
        v.put('M',1000); v.put('D',500); v.put('C',100); v.put('L',50);
        v.put('X',10); v.put('V',5); v.put('I',1);
        int total = 0;
        for (int i = 0; i < s.length(); i++) {
            if (i + 1 < s.length() && v.get(s.charAt(i)) < v.get(s.charAt(i+1))) total -= v.get(s.charAt(i));
            else total += v.get(s.charAt(i));
        }
        return total;
    }

    public static void main(String[] a) {
        System.out.println(romanToInt("XIV"));
    }
}
