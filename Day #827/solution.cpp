// Day 827: Min broadcast range.
// Sort towers; for each listener binary-search nearest tower, take min distance;
// answer = max over listeners. Time O((N+M) log M), Space O(1).
#include <bits/stdc++.h>
using namespace std;

long long minBroadcastRange(vector<long long> listeners, vector<long long> towers) {
    sort(towers.begin(), towers.end());
    long long ans = 0;
    for (long long l : listeners) {
        long long best = LLONG_MAX;
        auto it = lower_bound(towers.begin(), towers.end(), l);
        if (it != towers.end()) best = min(best, *it - l);
        if (it != towers.begin()) best = min(best, l - *prev(it));
        ans = max(ans, best);
    }
    return ans;
}

int main() {
    cout << minBroadcastRange({1, 5, 11, 20}, {4, 8, 15}) << "\n";
    return 0;
}
