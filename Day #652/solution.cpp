// Road-trip playlist: DP (LeetCode 920). dp[i][j]=playlists of length i with j distinct songs.
// dp[i][j]=dp[i-1][j-1]*(M-(j-1)) + dp[i-1][j]*max(j-B,0). Time O(N*M), space O(M).
#include <bits/stdc++.h>
using namespace std;
const long long MOD = 1000000007LL;

long long countPlaylists(int N, int M, int B) {
    vector<long long> prev(M + 1, 0), cur(M + 1, 0);
    prev[0] = 1;
    for (int i = 1; i <= N; ++i) {
        fill(cur.begin(), cur.end(), 0);
        for (int j = 1; j <= M; ++j) {
            cur[j] = (prev[j - 1] * (M - (j - 1))) % MOD;
            cur[j] = (cur[j] + prev[j] * max(j - B, 0)) % MOD;
        }
        swap(prev, cur);
    }
    return prev[M];
}

int main() {
    int N = 3, M = 3, B = 1;
    cout << countPlaylists(N, M, B) << "\n";
    return 0;
}
