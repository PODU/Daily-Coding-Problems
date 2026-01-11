// Min palindrome partition via DP with palindrome table + reconstruction.
// Time O(n^2), Space O(n^2).
import java.util.*;

public class Solution {
    static List<String> minPalindromePartition(String s){
        int n = s.length();
        List<String> res = new ArrayList<>();
        if(n == 0) return res;
        boolean[][] pal = new boolean[n][n];
        for(int i = n - 1; i >= 0; i--)
            for(int j = i; j < n; j++)
                if(s.charAt(i) == s.charAt(j) && (j - i < 2 || pal[i + 1][j - 1]))
                    pal[i][j] = true;

        int[] dp = new int[n + 1];
        int[] cut = new int[n + 1];
        Arrays.fill(dp, Integer.MAX_VALUE);
        Arrays.fill(cut, -1);
        dp[0] = 0;
        for(int i = 1; i <= n; i++)
            for(int j = 0; j < i; j++)
                if(pal[j][i - 1] && dp[j] != Integer.MAX_VALUE && dp[j] + 1 < dp[i]){
                    dp[i] = dp[j] + 1;
                    cut[i] = j;
                }

        for(int i = n; i > 0; i = cut[i]) res.add(s.substring(cut[i], i));
        Collections.reverse(res);
        return res;
    }

    static void print(List<String> v){
        StringBuilder sb = new StringBuilder("[");
        for(int i = 0; i < v.size(); i++){
            sb.append("\"").append(v.get(i)).append("\"");
            if(i + 1 < v.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }

    public static void main(String[] args){
        print(minPalindromePartition("racecarannakayak"));
        print(minPalindromePartition("abc"));
    }
}
