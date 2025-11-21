// Day 639: Letter combinations of a phone number.
// Approach: iterative Cartesian product over digit->letters map.
// Time: O(4^n * n), Space: O(4^n).
import java.util.*;

public class Solution {
    static List<String> letterCombinations(String digits, Map<Character, String[]> m) {
        List<String> res = new ArrayList<>();
        if (digits.isEmpty()) return res;
        res.add("");
        for (char d : digits.toCharArray()) {
            List<String> next = new ArrayList<>();
            for (String prefix : res)
                for (String ch : m.get(d))
                    next.add(prefix + ch);
            res = next;
        }
        return res;
    }

    public static void main(String[] args) {
        Map<Character, String[]> m = new HashMap<>();
        m.put('2', new String[]{"a","b","c"});
        m.put('3', new String[]{"d","e","f"});
        m.put('4', new String[]{"g","h","i"});
        m.put('5', new String[]{"j","k","l"});
        m.put('6', new String[]{"m","n","o"});
        m.put('7', new String[]{"p","q","r","s"});
        m.put('8', new String[]{"t","u","v"});
        m.put('9', new String[]{"w","x","y","z"});
        System.out.println(letterCombinations("23", m));
    }
}
