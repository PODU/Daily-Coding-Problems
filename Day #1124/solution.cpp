// Day 1124 - Minimum points to stab all intervals
// Greedy: sort by right endpoint, place a point at the end of each not-yet-
// stabbed interval. Time: O(n log n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> stab(vector<pair<int,int>> intervals) {
    sort(intervals.begin(), intervals.end(),
         [](auto& a, auto& b) { return a.second < b.second; });
    vector<int> points;
    long long last = LLONG_MIN;
    for (auto& [s, e] : intervals)
        if (s > last) { last = e; points.push_back(e); }
    return points;
}

int main() {
    auto pts = stab({{1, 4}, {4, 5}, {7, 9}, {9, 12}});
    cout << "[";
    for (size_t i = 0; i < pts.size(); ++i)
        cout << pts[i] << (i + 1 < pts.size() ? ", " : "");
    cout << "]\n"; // [4, 9]
    return 0;
}
