// Phone keypad letter combinations via backtracking. O(prod of choices) time.
import java.util.*;

public class Solution {
    static final Map<Character, String> M = new HashMap<>();
    static {
        M.put('2', "abc"); M.put('3', "def"); M.put('4', "ghi");
        M.put('5', "jkl"); M.put('6', "mno"); M.put('7', "pqrs");
        M.put('8', "tuv"); M.put('9', "wxyz");
    }

    static void backtrack(String digits, int i, StringBuilder cur, List<String> out) {
        if (i == digits.length()) {
            if (digits.length() > 0) out.add(cur.toString());
            return;
        }
        for (char c : M.get(digits.charAt(i)).toCharArray()) {
            cur.append(c);
            backtrack(digits, i + 1, cur, out);
            cur.deleteCharAt(cur.length() - 1);
        }
    }

    public static void main(String[] args) {
        String digits = "23";
        List<String> out = new ArrayList<>();
        backtrack(digits, 0, new StringBuilder(), out);
        System.out.println("[" + String.join(", ", out) + "]");
    }
}
