// Min broadcast range: sort towers, binary-search nearest tower per listener, take max.
// Time O((N+M) log M), Space O(1) extra.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> listeners = {1, 5, 11, 20};
    vector<int> towers = {4, 8, 15};
    sort(towers.begin(), towers.end());
    int ans = 0;
    for (int L : listeners) {
        auto it = lower_bound(towers.begin(), towers.end(), L);
        int best = INT_MAX;
        if (it != towers.end()) best = min(best, *it - L);
        if (it != towers.begin()) best = min(best, L - *prev(it));
        ans = max(ans, best);
    }
    cout << ans << "\n";
    return 0;
}
