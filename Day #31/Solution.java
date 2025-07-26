// Edit Distance via DP. Time O(m*n), Space O(min(m,n)) using two rolling rows.
public class Solution {
    static int editDistance(String a, String b) {
        if (a.length() < b.length()) { String t = a; a = b; b = t; }
        int m = a.length(), n = b.length();
        int[] prev = new int[n + 1], cur = new int[n + 1];
        for (int j = 0; j <= n; j++) prev[j] = j;
        for (int i = 1; i <= m; i++) {
            cur[0] = i;
            for (int j = 1; j <= n; j++) {
                if (a.charAt(i - 1) == b.charAt(j - 1)) cur[j] = prev[j - 1];
                else cur[j] = 1 + Math.min(prev[j - 1], Math.min(prev[j], cur[j - 1]));
            }
            int[] t = prev; prev = cur; cur = t;
        }
        return prev[n];
    }

    public static void main(String[] args) {
        System.out.println(editDistance("kitten", "sitting"));
    }
}
