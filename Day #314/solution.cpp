// Day 314: Min broadcast range = max over listeners of distance to nearest tower.
// Sort towers, binary search each listener. O((N+M) log M).
#include <bits/stdc++.h>
using namespace std;
int minRange(vector<int> listeners, vector<int> towers) {
    sort(towers.begin(), towers.end());
    int ans = 0;
    for (int L : listeners) {
        auto it = lower_bound(towers.begin(), towers.end(), L);
        int best = INT_MAX;
        if (it != towers.end()) best = min(best, *it - L);
        if (it != towers.begin()) best = min(best, L - *prev(it));
        ans = max(ans, best);
    }
    return ans;
}
int main() {
    cout << minRange({1, 5, 11, 20}, {4, 8, 15}) << "\n"; // 5
}
