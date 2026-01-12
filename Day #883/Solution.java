// Min steps to 1: dp[i] = 1 + min(dp[i-1], dp[i/a] over factors a). Time O(N*sqrt N), Space O(N).
public class Solution {
    static int minSteps(int n){
        int[] dp = new int[n + 1];
        for(int i = 2; i <= n; i++){
            dp[i] = dp[i - 1] + 1;
            for(int a = 2; (long) a * a <= i; a++){
                if(i % a == 0){
                    int larger = i / a;
                    dp[i] = Math.min(dp[i], 1 + dp[larger]);
                }
            }
        }
        return dp[n];
    }

    public static void main(String[] args){
        System.out.println(minSteps(100));
    }
}
