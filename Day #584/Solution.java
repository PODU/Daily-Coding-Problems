// Greedy rearrange: at each step pick highest remaining count != prev char, tie by smallest char.
// Feasible iff maxCount <= (n+1)/2. Time O(n*26), Space O(26).
public class Solution {
    static String rearrange(String s) {
        int[] cnt = new int[26];
        for (char c : s.toCharArray()) cnt[c - 'a']++;
        int n = s.length();
        StringBuilder res = new StringBuilder();
        int prev = -1;
        for (int k = 0; k < n; k++) {
            int best = -1;
            for (int i = 0; i < 26; i++) {
                if (i == prev) continue;
                if (cnt[i] <= 0) continue;
                if (best == -1 || cnt[i] > cnt[best]) best = i;
            }
            if (best == -1) return null;
            res.append((char) ('a' + best));
            cnt[best]--;
            prev = best;
        }
        return res.toString();
    }

    public static void main(String[] args) {
        String a = rearrange("aaabbc");
        System.out.println(a == null ? "None" : a);
        String b = rearrange("aaab");
        System.out.println(b == null ? "None" : b);
    }
}
