// Day 957: skyline problem via sweep line with a multiset of active heights.
// Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<pair<int,int>> skyline(vector<array<int,3>> buildings) {
    vector<pair<int,int>> events; // (x, signed height): negative = start
    for (auto& b : buildings) {
        events.push_back({b[0], -b[2]});
        events.push_back({b[1], b[2]});
    }
    sort(events.begin(), events.end());
    multiset<int> active = {0};
    int prev = 0;
    vector<pair<int,int>> res;
    for (size_t i = 0; i < events.size(); ++i) {
        int x = events[i].first, h = events[i].second;
        if (h < 0) active.insert(-h);
        else active.erase(active.find(h));
        int cur = *active.rbegin();
        if (cur != prev) { res.push_back({x, cur}); prev = cur; }
    }
    return res;
}

int main() {
    auto res = skyline({{0,15,3},{4,11,5},{19,23,4}});
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << "(" << res[i].first << ", " << res[i].second << ")" << (i + 1 < res.size() ? ", " : "");
    cout << "]\n"; // [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
    return 0;
}
