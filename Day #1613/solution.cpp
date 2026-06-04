// Skyline via sweep-line + max-heap (multiset). Emit key point when max height changes.
// Time: O(n log n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<array<int,3>> buildings = {{0,15,3},{4,11,5},{19,23,4}};
    vector<pair<int,int>> events; // (x, signed height): start = -h, end = +h
    for (auto& b : buildings) {
        events.push_back({b[0], -b[2]});
        events.push_back({b[1],  b[2]});
    }
    sort(events.begin(), events.end());

    multiset<int> heights = {0};
    int prev = 0;
    vector<pair<int,int>> res;
    for (auto& e : events) {
        int x = e.first, h = e.second;
        if (h < 0) heights.insert(-h);
        else       heights.erase(heights.find(h));
        int cur = *heights.rbegin();
        if (cur != prev) {
            res.push_back({x, cur});
            prev = cur;
        }
    }

    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << "(" << res[i].first << ", " << res[i].second << ")";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
