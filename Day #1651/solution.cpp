// Day 1651: Count valid playlists of length N from M songs, each used >=1 time,
// with a buffer of B songs between repeats.
// DP: dp[i][j] = #playlists of length i using exactly j distinct songs.
// Time O(N*M), Space O(N*M).
#include <bits/stdc++.h>
using namespace std;
const long long MOD = 1000000007LL;

long long numPlaylists(int N, int M, int B) {
    vector<vector<long long>> dp(N + 1, vector<long long>(M + 1, 0));
    dp[0][0] = 1;
    for (int i = 1; i <= N; i++)
        for (int j = 1; j <= M; j++) {
            // place a brand new song: (M-(j-1)) choices
            dp[i][j] = dp[i - 1][j - 1] * (M - (j - 1)) % MOD;
            // reuse an old song that is far enough back: max(j-B,0) choices
            dp[i][j] = (dp[i][j] + dp[i - 1][j] * max(j - B, 0)) % MOD;
        }
    return dp[N][M];
}

int main() {
    // N=3 songs, M=2 downloaded, B=0 buffer -> 6 valid playlists
    cout << numPlaylists(3, 2, 0) << "\n";
    return 0;
}
