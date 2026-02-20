// Day 1104: Phone digit -> letters combinations via backtracking.
// Time: O(prod of choices * len). Space: O(len) recursion.
import java.util.*;

public class Solution {
    static void dfs(String digits, int i, StringBuilder cur,
                    Map<Character,char[]> mp, List<String> out){
        if (i == digits.length()) { out.add(cur.toString()); return; }
        for (char c : mp.get(digits.charAt(i))){
            cur.append(c);
            dfs(digits, i+1, cur, mp, out);
            cur.deleteCharAt(cur.length()-1);
        }
    }
    static List<String> letterCombos(String digits, Map<Character,char[]> mp){
        List<String> out = new ArrayList<>();
        if (digits.isEmpty()) return out;
        dfs(digits, 0, new StringBuilder(), mp, out);
        return out;
    }
    public static void main(String[] args){
        Map<Character,char[]> mp = new HashMap<>();
        mp.put('2', new char[]{'a','b','c'});
        mp.put('3', new char[]{'d','e','f'});
        System.out.println(letterCombos("23", mp)); // [ad, ae, af, bd, be, bf, cd, ce, cf]
    }
}
