// Day 1334: Levenshtein edit distance between two strings.
// Classic DP with rolling row. O(n*m) time, O(min(n,m)) space.
public class Solution {
    static int editDistance(String a, String b) {
        int n = a.length(), m = b.length();
        int[] prev = new int[m + 1], cur = new int[m + 1];
        for (int j = 0; j <= m; j++) prev[j] = j;
        for (int i = 1; i <= n; i++) {
            cur[0] = i;
            for (int j = 1; j <= m; j++) {
                if (a.charAt(i - 1) == b.charAt(j - 1)) cur[j] = prev[j - 1];
                else cur[j] = 1 + Math.min(prev[j - 1], Math.min(prev[j], cur[j - 1]));
            }
            int[] t = prev; prev = cur; cur = t;
        }
        return prev[m];
    }

    public static void main(String[] args) {
        System.out.println(editDistance("kitten", "sitting")); // 3
    }
}
