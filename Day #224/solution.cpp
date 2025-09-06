// Day 224: Smallest positive integer not expressible as a subset sum (sorted array).
// Approach: greedy. Keep reachable range [1, ans-1]; if next a <= ans, extend by a, else ans is the gap.
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long smallestNonSubsetSum(const vector<long long>& a) {
    long long ans = 1; // smallest unreachable so far
    for (long long x : a) {
        if (x > ans) break;
        ans += x;
    }
    return ans;
}

int main() {
    cout << smallestNonSubsetSum({1, 2, 3, 10}) << endl; // 7
    return 0;
}
