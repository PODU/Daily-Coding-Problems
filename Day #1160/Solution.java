// Recover digits from scrambled English spellings using unique identifying letters.
// Time: O(L), Space: O(1).
public class Solution {
    static String recover(String s) {
        int[] c = new int[26];
        for (char ch : s.toCharArray()) c[ch - 'a']++;
        int[] cnt = new int[10];
        cnt[0] = c['z' - 'a'];
        cnt[2] = c['w' - 'a'];
        cnt[4] = c['u' - 'a'];
        cnt[6] = c['x' - 'a'];
        cnt[8] = c['g' - 'a'];
        cnt[3] = c['h' - 'a'] - cnt[8];
        cnt[5] = c['f' - 'a'] - cnt[4];
        cnt[7] = c['s' - 'a'] - cnt[6];
        cnt[1] = c['o' - 'a'] - cnt[0] - cnt[2] - cnt[4];
        cnt[9] = c['i' - 'a'] - cnt[5] - cnt[6] - cnt[8];
        StringBuilder sb = new StringBuilder();
        for (int d = 0; d <= 9; d++)
            for (int k = 0; k < cnt[d]; k++) sb.append((char) ('0' + d));
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(recover("niesevehrtfeev"));
    }
}
