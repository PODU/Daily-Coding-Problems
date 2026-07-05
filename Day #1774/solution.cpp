// Day 1774: Fewest steps to reduce N to 1 (decrement by 1, or jump to the larger
// factor of any factorization). DP over 1..N, trying every divisor. O(N*sqrt N) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

int minSteps(int N) {
    vector<int> dp(N + 1, 0);
    for (int i = 2; i <= N; ++i) {
        dp[i] = dp[i - 1] + 1;               // decrement step
        for (int a = 2; (long long)a * a <= i; ++a) {
            if (i % a == 0) dp[i] = min(dp[i], dp[i / a] + 1); // jump to larger factor i/a
        }
    }
    return dp[N];
}

int main() {
    cout << minSteps(100) << "\n"; // 100->10->9->3->2->1 = 5
    return 0;
}
