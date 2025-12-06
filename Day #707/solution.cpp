// Day 707: Min broadcast range. Sort towers; for each listener binary-search the
// nearest tower, answer is max of those min distances. Time O((N+M)logM).
#include <bits/stdc++.h>
using namespace std;

int minRange(vector<int> listeners, vector<int> towers) {
    sort(towers.begin(), towers.end());
    int ans = 0;
    for (int x : listeners) {
        auto it = lower_bound(towers.begin(), towers.end(), x);
        int best = INT_MAX;
        if (it != towers.end()) best = min(best, *it - x);
        if (it != towers.begin()) best = min(best, x - *prev(it));
        ans = max(ans, best);
    }
    return ans;
}

int main() {
    cout << minRange({1, 5, 11, 20}, {4, 8, 15}) << endl;
    return 0;
}
