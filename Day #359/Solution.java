// Recover digits from scrambled letters: use unique marker letters (z,w,u,x,g) then derive the rest. O(N) time, O(1) space.
public class Solution {
    static String recover(String s) {
        int[] c = new int[26];
        for (char ch : s.toCharArray())
            if (ch >= 'a' && ch <= 'z') c[ch - 'a']++;
        int[] d = new int[10];
        d[0] = c['z' - 'a'];
        d[2] = c['w' - 'a'];
        d[4] = c['u' - 'a'];
        d[6] = c['x' - 'a'];
        d[8] = c['g' - 'a'];
        d[3] = c['h' - 'a'] - d[8];
        d[5] = c['f' - 'a'] - d[4];
        d[7] = c['s' - 'a'] - d[6];
        d[1] = c['o' - 'a'] - d[0] - d[2] - d[4];
        d[9] = c['i' - 'a'] - d[5] - d[6] - d[8];
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < 10; i++)
            for (int k = 0; k < d[i]; k++) sb.append((char) ('0' + i));
        return sb.toString();
    }
    public static void main(String[] args) {
        System.out.println(recover("niesevehrtfeev"));
    }
}
