// Edit distance (Levenshtein) via DP. Time O(n*m), Space O(min(n,m)).
public class Solution {
    static int editDistance(String a, String b) {
        String s = a.length() <= b.length() ? a : b;
        String t = a.length() <= b.length() ? b : a;
        int n = s.length(), m = t.length();
        int[] prev = new int[n + 1], cur = new int[n + 1];
        for (int i = 0; i <= n; i++) prev[i] = i;
        for (int j = 1; j <= m; j++) {
            cur[0] = j;
            for (int i = 1; i <= n; i++) {
                int cost = s.charAt(i - 1) == t.charAt(j - 1) ? 0 : 1;
                cur[i] = Math.min(Math.min(prev[i] + 1, cur[i - 1] + 1), prev[i - 1] + cost);
            }
            int[] tmp = prev; prev = cur; cur = tmp;
        }
        return prev[n];
    }

    public static void main(String[] args) {
        System.out.println(editDistance("kitten", "sitting"));
    }
}
