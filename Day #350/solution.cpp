// Min perfect squares summing to N via DP, then greedy-largest reconstruction.
// Time O(N*sqrt N), Space O(N).
#include <iostream>
#include <vector>
#include <string>
using namespace std;

string solve(int n) {
    vector<int> dp(n + 1, 1e9);
    dp[0] = 0;
    for (int i = 1; i <= n; i++)
        for (int s = 1; s * s <= i; s++)
            dp[i] = min(dp[i], dp[i - s * s] + 1);

    // Reconstruct: greedily pick largest s with dp[i - s*s] == dp[i] - 1.
    vector<int> squares;
    int i = n;
    while (i > 0) {
        for (int s = (int)(__builtin_sqrt((double)i)); s >= 1; s--) {
            if (s * s <= i && dp[i - s * s] == dp[i] - 1) {
                squares.push_back(s * s);
                i -= s * s;
                break;
            }
        }
    }

    string out = to_string(dp[n]) + " (";
    for (size_t k = 0; k < squares.size(); k++) {
        if (k) out += " + ";
        out += to_string(squares[k]);
    }
    out += ")";
    return out;
}

int main() {
    for (int n : {4, 17, 18}) cout << solve(n) << "\n";
    return 0;
}
