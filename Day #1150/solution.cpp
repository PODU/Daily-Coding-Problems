// Day 1150: Skyline - sweep line over building edges with a height multiset.
// Track current max height; emit point when it changes. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<pair<int,int>> skyline(vector<tuple<int,int,int>>& bld) {
    vector<pair<int,int>> events; // (x, signedHeight): start=-h, end=+h
    for (size_t i = 0; i < bld.size(); ++i) {
        int l = get<0>(bld[i]), r = get<1>(bld[i]), h = get<2>(bld[i]);
        events.push_back(make_pair(l, -h));
        events.push_back(make_pair(r, h));
    }
    sort(events.begin(), events.end());
    multiset<int> heights;
    heights.insert(0);
    int prev = 0;
    vector<pair<int,int>> res;
    for (size_t i = 0; i < events.size(); ++i) {
        int x = events[i].first, h = events[i].second;
        if (h < 0) heights.insert(-h);
        else heights.erase(heights.find(h));
        int cur = *heights.rbegin();
        if (cur != prev) { res.push_back(make_pair(x, cur)); prev = cur; }
    }
    return res;
}

int main() {
    vector<tuple<int,int,int>> bld = {{0, 15, 3}, {4, 11, 5}, {19, 23, 4}};
    auto sk = skyline(bld);
    cout << "[";
    for (size_t i = 0; i < sk.size(); ++i)
        cout << "(" << sk[i].first << ", " << sk[i].second << ")" << (i + 1 < sk.size() ? ", " : "");
    cout << "]\n"; // [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
    return 0;
}
