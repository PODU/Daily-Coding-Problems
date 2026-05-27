// Phone keypad letter combinations via iterative Cartesian product. O(prod*len) time, O(output) space.
import java.util.*;

public class Solution {
    static List<String> letterCombinations(String digits, Map<Character, String> mp) {
        List<String> res = new ArrayList<>();
        if (digits.isEmpty()) return res;
        res.add("");
        for (char d : digits.toCharArray()) {
            List<String> next = new ArrayList<>();
            for (String pre : res)
                for (char c : mp.get(d).toCharArray()) next.add(pre + c);
            res = next;
        }
        return res;
    }

    public static void main(String[] args) {
        Map<Character, String> mp = new HashMap<>();
        mp.put('2', "abc");
        mp.put('3', "def");
        List<String> res = letterCombinations("23", mp);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++)
            sb.append("\"").append(res.get(i)).append("\"").append(i + 1 < res.size() ? ", " : "");
        sb.append("]");
        System.out.println(sb.toString());
    }
}
