// Day 767: Find all start indices in S that are anagrams of W.
// Sliding window of size |W| with a 26-count match counter. O(|S|) time, O(1) space.
import java.util.*;

public class Solution {
    static List<Integer> findAnagrams(String s, String w) {
        List<Integer> res = new ArrayList<>();
        int n = s.length(), m = w.length();
        if (m > n) return res;
        int[] need = new int[26], win = new int[26];
        for (char c : w.toCharArray()) need[c - 'a']++;
        for (int i = 0; i < n; i++) {
            win[s.charAt(i) - 'a']++;
            if (i >= m) win[s.charAt(i - m) - 'a']--;
            if (i >= m - 1 && Arrays.equals(need, win)) res.add(i - m + 1);
        }
        return res;
    }

    public static void main(String[] args) {
        List<Integer> r = findAnagrams("abxaba", "ab");
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < r.size(); i++) {
            sb.append(r.get(i));
            if (i + 1 < r.size()) sb.append(", ");
        }
        System.out.println(sb); // 0, 3, 4
    }
}
