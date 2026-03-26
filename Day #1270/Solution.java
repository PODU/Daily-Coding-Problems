// Day 1270: Find all start indices in S that are anagrams of W.
// Fixed-size sliding window with a 26-length count. O(|S|) time.
import java.util.*;

public class Solution {
    static List<Integer> findAnagrams(String w, String s) {
        List<Integer> res = new ArrayList<>();
        int m = w.length(), n = s.length();
        if (m > n) return res;
        int[] need = new int[26], win = new int[26];
        for (char c : w.toCharArray()) need[c - 'a']++;
        for (int i = 0; i < n; i++) {
            win[s.charAt(i) - 'a']++;
            if (i >= m) win[s.charAt(i - m) - 'a']--;
            if (i >= m - 1 && Arrays.equals(win, need)) res.add(i - m + 1);
        }
        return res;
    }

    public static void main(String[] args) {
        List<Integer> res = findAnagrams("ab", "abxaba");
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < res.size(); i++) {
            sb.append(res.get(i));
            if (i + 1 < res.size()) sb.append(", ");
        }
        System.out.println(sb);
    }
}
