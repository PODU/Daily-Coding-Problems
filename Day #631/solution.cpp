// Day 631: Skyline problem.
// Approach: sweep line over building edges + multiset of active heights.
// Time: O(n log n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

vector<pair<int,int>> getSkyline(vector<array<int,3>>& buildings) {
    vector<pair<int,int>> events; // (x, height): negative height = start, positive = end
    for (auto& b : buildings) {
        events.push_back({b[0], -b[2]});
        events.push_back({b[1], b[2]});
    }
    sort(events.begin(), events.end());
    multiset<int> heights = {0};
    int prev = 0;
    vector<pair<int,int>> res;
    for (auto& e : events) {
        int x = e.first, h = e.second;
        if (h < 0) heights.insert(-h);
        else heights.erase(heights.find(h));
        int cur = *heights.rbegin();
        if (cur != prev) {
            res.push_back({x, cur});
            prev = cur;
        }
    }
    return res;
}

int main() {
    vector<array<int,3>> buildings = {{0,15,3},{4,11,5},{19,23,4}};
    auto sky = getSkyline(buildings);
    cout << "[";
    for (size_t i = 0; i < sky.size(); i++) {
        cout << "(" << sky[i].first << ", " << sky[i].second << ")";
        if (i + 1 < sky.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
