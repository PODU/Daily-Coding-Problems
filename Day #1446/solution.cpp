// Day 1446: Minimum set of points hitting every closed interval.
// Greedy: sort by right endpoint; whenever the current interval is unhit, pick
// its right endpoint. Time O(n log n), Space O(1). (Any minimal set is valid.)
#include <bits/stdc++.h>
using namespace std;

vector<int> minStabbingSet(vector<pair<int,int>> intervals) {
    sort(intervals.begin(), intervals.end(),
         [](auto& a, auto& b){ return a.second < b.second; });
    vector<int> points;
    long long last = LLONG_MIN;
    for (size_t i = 0; i < intervals.size(); i++) {
        int l = intervals[i].first, r = intervals[i].second;
        if (l > last) { points.push_back(r); last = r; }
    }
    return points;
}

int main() {
    vector<pair<int,int>> intervals = {{0,3},{2,6},{3,4},{6,9}};
    vector<int> pts = minStabbingSet(intervals);
    cout << "{";
    for (size_t i = 0; i < pts.size(); i++)
        cout << pts[i] << (i + 1 < pts.size() ? ", " : "");
    cout << "}\n"; // e.g. {3, 9} -- a valid minimum (so is {3, 6})
    return 0;
}
