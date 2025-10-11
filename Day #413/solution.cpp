// Day 413: Ordered ways to climb a staircase with allowed step sizes X.
// DP: ways[n] = sum over x in X of ways[n-x]. Time O(N*|X|), Space O(N).
#include <bits/stdc++.h>
using namespace std;

long long countWays(int n, const vector<int>& steps) {
    vector<long long> ways(n + 1, 0);
    ways[0] = 1;
    for (int i = 1; i <= n; ++i)
        for (int x : steps)
            if (x <= i) ways[i] += ways[i - x];
    return ways[n];
}

int main() {
    cout << countWays(4, {1, 2}) << "\n";          // 5
    cout << countWays(10, {1, 3, 5}) << "\n";       // generalized X={1,3,5}
    return 0;
}
