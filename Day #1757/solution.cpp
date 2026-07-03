// Day 1757: Count ordered ways to climb N stairs (steps 1 or 2 -> Fibonacci).
// Generalized to a step set X via DP: ways[i] = sum of ways[i-x]. O(N*|X|) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

long long climbWays(int n, const vector<int>& steps) {
    vector<long long> ways(n + 1, 0);
    ways[0] = 1;
    for (int i = 1; i <= n; ++i)
        for (int x : steps)
            if (i - x >= 0) ways[i] += ways[i - x];
    return ways[n];
}

int main() {
    int N = 4;
    cout << climbWays(N, {1, 2}) << "\n"; // 5
    cout << "Generalized X={1,3,5}, N=4: " << climbWays(N, {1, 3, 5}) << "\n";
    return 0;
}
