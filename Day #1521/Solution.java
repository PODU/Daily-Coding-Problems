// Find all anagram start indices of W in S.
// Sliding window + 26-letter freq + match counter. Time O(|S|), Space O(1).
import java.util.*;

public class Solution {
    static List<Integer> findAnagrams(String W, String S) {
        List<Integer> res = new ArrayList<>();
        int n = S.length(), m = W.length();
        if (m == 0 || m > n) return res;
        int[] need = new int[26], win = new int[26];
        for (char c : W.toCharArray()) need[c - 'a']++;
        int matches = 0;
        for (int i = 0; i < 26; i++) if (need[i] == 0) matches++;
        for (int i = 0; i < n; i++) {
            int add = S.charAt(i) - 'a';
            win[add]++;
            if (win[add] == need[add]) matches++;
            else if (win[add] == need[add] + 1) matches--;
            if (i >= m) {
                int rem = S.charAt(i - m) - 'a';
                win[rem]--;
                if (win[rem] == need[rem]) matches++;
                else if (win[rem] == need[rem] - 1) matches--;
            }
            if (i >= m - 1 && matches == 26) res.add(i - m + 1);
        }
        return res;
    }

    public static void main(String[] args) {
        List<Integer> idx = findAnagrams("ab", "abxaba");
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < idx.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append(idx.get(i));
        }
        System.out.println(sb.toString());
    }
}
