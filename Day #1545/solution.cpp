// Pyramid min cost (only lowering). For each index the max pyramid height is
// min of a left pass (rise +1 from edge) and a right pass. A pyramid of peak h
// retains h*h stones, so cost = sum(a) - max(h)^2. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

long long minCost(const vector<int>& a) {
    int n = a.size();
    vector<long long> left(n), right(n);
    left[0] = min(a[0], 1);
    for (int i = 1; i < n; i++) left[i] = min((long long)a[i], left[i-1] + 1);
    right[n-1] = min(a[n-1], 1);
    for (int i = n-2; i >= 0; i--) right[i] = min((long long)a[i], right[i+1] + 1);
    long long bestPeak = 0, sum = 0;
    for (int i = 0; i < n; i++) {
        bestPeak = max(bestPeak, min(left[i], right[i]));
        sum += a[i];
    }
    return sum - bestPeak * bestPeak;
}

int main() {
    vector<int> a = {1, 1, 3, 3, 2, 1};
    cout << minCost(a) << "\n";
    return 0;
}
