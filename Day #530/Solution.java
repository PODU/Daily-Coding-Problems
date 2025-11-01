// Levenshtein edit distance via DP with rolling array.
// Time O(m*n), Space O(min(m,n)).
public class Solution {
    static int editDistance(String a, String b) {
        if (a.length() < b.length()) return editDistance(b, a);
        int n = b.length();
        int[] prev = new int[n + 1], cur = new int[n + 1];
        for (int j = 0; j <= n; j++) prev[j] = j;
        for (int i = 1; i <= a.length(); i++) {
            cur[0] = i;
            for (int j = 1; j <= n; j++) {
                int cost = a.charAt(i - 1) == b.charAt(j - 1) ? 0 : 1;
                cur[j] = Math.min(Math.min(prev[j] + 1, cur[j - 1] + 1), prev[j - 1] + cost);
            }
            int[] t = prev; prev = cur; cur = t;
        }
        return prev[n];
    }

    public static void main(String[] args) {
        System.out.println(editDistance("kitten", "sitting"));
    }
}
