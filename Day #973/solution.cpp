// Day 973: Count ways to decode a digit string (a=1..z=26).
// Approach: DP, dp[i]=dp[i-1] if 1-digit valid + dp[i-2] if 2-digit valid. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int numDecodings(const string& s) {
    if (s.empty() || s[0] == '0') return 0;
    int prev2 = 1, prev1 = 1; // dp[0]=1, dp[1]=1
    for (int i = 1; i < (int)s.size(); ++i) {
        int cur = 0;
        if (s[i] != '0') cur += prev1;
        int two = (s[i-1] - '0') * 10 + (s[i] - '0');
        if (two >= 10 && two <= 26) cur += prev2;
        prev2 = prev1;
        prev1 = cur;
    }
    return prev1;
}

int main() {
    cout << numDecodings("111") << endl; // 3
    return 0;
}
