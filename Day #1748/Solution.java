// Day 1748: Recover digits from an anagram of concatenated number words (zero-nine).
// Approach: unique-letter signatures (z,w,u,x,g first; then derive odd digits). O(n) time, O(1) space.
public class Solution {
    static String recover(String s) {
        int[] c = new int[26];
        for (char ch : s.toCharArray()) c[ch - 'a']++;
        int[] cnt = new int[10];
        cnt[0] = c['z' - 'a'];                        // zero
        cnt[2] = c['w' - 'a'];                        // two
        cnt[4] = c['u' - 'a'];                        // four
        cnt[6] = c['x' - 'a'];                        // six
        cnt[8] = c['g' - 'a'];                        // eight
        cnt[3] = c['h' - 'a'] - cnt[8];               // three
        cnt[5] = c['f' - 'a'] - cnt[4];               // five
        cnt[7] = c['s' - 'a'] - cnt[6];               // seven
        cnt[1] = c['o' - 'a'] - cnt[0] - cnt[2] - cnt[4]; // one
        cnt[9] = c['i' - 'a'] - cnt[5] - cnt[6] - cnt[8]; // nine

        StringBuilder sb = new StringBuilder();
        for (int d = 0; d <= 9; d++)
            for (int k = 0; k < cnt[d]; k++) sb.append((char) ('0' + d));
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(recover("niesevehrtfeev")); // 357
    }
}
