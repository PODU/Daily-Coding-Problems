// Min steps to reduce N to 1 (N-1 or replace with max factor): DP.
// Time O(N*sqrt(N)), Space O(N).
public class Solution {
    static int minSteps(int N){
        int[] dp = new int[N + 1];
        for(int n = 2; n <= N; n++){
            dp[n] = dp[n - 1] + 1;
            for(int a = 2; (long) a * a <= n; a++){
                if(n % a == 0){
                    int b = n / a;      // b >= a, max(a,b)=b
                    dp[n] = Math.min(dp[n], dp[b] + 1);
                }
            }
        }
        return dp[N];
    }

    public static void main(String[] args){
        System.out.println(minSteps(100));
    }
}
