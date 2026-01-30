// Day 990: Count ordered ways to climb N steps using step sizes from set X.
// Bottom-up DP: ways[i] = sum over x in X of ways[i-x]. O(N*|X|) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

long long climbWays(int n, const vector<int>& X) {
    vector<long long> ways(n + 1, 0);
    ways[0] = 1;
    for (int i = 1; i <= n; i++)
        for (int x : X)
            if (i - x >= 0) ways[i] += ways[i - x];
    return ways[n];
}

int main() {
    cout << "N=4, X={1,2}: " << climbWays(4, {1, 2}) << endl;     // expected 5
    cout << "N=4, X={1,3,5}: " << climbWays(4, {1, 3, 5}) << endl; // generalized
    return 0;
}
