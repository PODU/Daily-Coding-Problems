// Day 111: Anagram substrings via fixed sliding window of char counts. O(|S|).
import java.util.*;

public class Solution {
    static List<Integer> findAnagrams(String s, String w){
        List<Integer> res = new ArrayList<>();
        int n = s.length(), m = w.length();
        if (m > n) return res;
        int[] need = new int[256], win = new int[256];
        for (char c : w.toCharArray()) need[c]++;
        for (int i = 0; i < n; i++){
            win[s.charAt(i)]++;
            if (i >= m) win[s.charAt(i - m)]--;
            if (i >= m - 1 && Arrays.equals(need, win)) res.add(i - m + 1);
        }
        return res;
    }

    public static void main(String[] args){
        List<Integer> r = findAnagrams("abxaba", "ab");
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < r.size(); i++){
            sb.append(r.get(i));
            if (i + 1 < r.size()) sb.append(", ");
        }
        System.out.println(sb.toString());
    }
}
