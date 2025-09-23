// Count playlists of length N from M songs, each used >=1, gap >=B between repeats.
// DP over length x distinct songs (LeetCode 920). Time O(N*M), Space O(N*M).
#include <bits/stdc++.h>
using namespace std;

const long long MOD = 1000000007LL;

long long numPlaylists(int N, int M, int B) {
    vector<vector<long long>> dp(N + 1, vector<long long>(M + 1, 0));
    dp[0][0] = 1;
    for (int i = 1; i <= N; ++i) {
        for (int j = 1; j <= M; ++j) {
            dp[i][j] = dp[i - 1][j - 1] * (M - (j - 1)) % MOD;
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * max(j - B, 0)) % MOD;
        }
    }
    return dp[N][M];
}

int main() {
    int N = 3, M = 3, B = 1;
    cout << "Number of valid playlists (N=" << N << ", M=" << M << ", B=" << B
         << ") = " << numPlaylists(N, M, B) << "\n";
    return 0;
}
