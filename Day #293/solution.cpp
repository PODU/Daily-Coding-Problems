// Pyramid reshape (lowering only): L[i]/R[i] cap ramp slopes, peak v=max min(L,R), cost=sum-v*v.
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

long long minCost(vector<int>& h) {
    int n = h.size();
    vector<long long> L(n), R(n);
    L[0] = min(h[0], 1);
    for (int i = 1; i < n; ++i) L[i] = min((long long)h[i], L[i-1] + 1);
    R[n-1] = min(h[n-1], 1);
    for (int i = n-2; i >= 0; --i) R[i] = min((long long)h[i], R[i+1] + 1);
    long long v = 0, sum = 0;
    for (int i = 0; i < n; ++i) { v = max(v, min(L[i], R[i])); sum += h[i]; }
    return sum - v * v;
}

int main() {
    vector<int> h = {1, 1, 3, 3, 2, 1};
    cout << minCost(h) << "\n";
    return 0;
}
