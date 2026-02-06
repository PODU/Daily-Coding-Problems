// Day 1033: Minimum subset-sum difference (partition into two subsets).
// Boolean subset-sum DP over reachable sums up to total/2; answer total-2*best. O(n*sum) time, O(sum) space.
#include <bits/stdc++.h>
using namespace std;

int minDiff(const vector<int>& a) {
    int total = accumulate(a.begin(), a.end(), 0);
    vector<char> dp(total / 2 + 1, 0);
    dp[0] = 1;
    for (int x : a)
        for (int s = total / 2; s >= x; --s)
            if (dp[s - x]) dp[s] = 1;
    for (int s = total / 2; s >= 0; --s)
        if (dp[s]) return total - 2 * s;
    return total;
}

int main() {
    vector<int> a = {5, 10, 15, 20, 25};
    cout << minDiff(a) << "\n";
}
