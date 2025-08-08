// Day 81: Phone-number letter combinations via iterative cartesian product.
// Time O(prod of letters * len), Space O(output).
import java.util.*;

public class Solution {
    static List<String> letterCombinations(Map<Character, String[]> mapping, String digits) {
        if (digits.isEmpty()) return new ArrayList<>();
        List<String> res = new ArrayList<>(List.of(""));
        for (char d : digits.toCharArray()) {
            List<String> next = new ArrayList<>();
            for (String prefix : res)
                for (String letter : mapping.get(d))
                    next.add(prefix + letter);
            res = next;
        }
        return res;
    }

    public static void main(String[] args) {
        Map<Character, String[]> mapping = new HashMap<>();
        mapping.put('2', new String[]{"a","b","c"});
        mapping.put('3', new String[]{"d","e","f"});
        List<String> res = letterCombinations(mapping, "23");
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++)
            sb.append("\"").append(res.get(i)).append("\"").append(i + 1 < res.size() ? ", " : "");
        sb.append("]");
        System.out.println(sb);
        // ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    }
}
