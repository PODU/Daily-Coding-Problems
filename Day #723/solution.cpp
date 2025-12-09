// Day 723: Smallest set of points stabbing all closed intervals.
// Approach: Sort by right endpoint; greedily pick the end of each uncovered interval.
// Time: O(n log n), Space: O(n). (Any minimum-size set is valid; {3,9} here.)
#include <bits/stdc++.h>
using namespace std;

vector<int> minStabbingPoints(vector<pair<int,int>> intervals) {
    sort(intervals.begin(), intervals.end(),
         [](auto& a, auto& b){ return a.second < b.second; });
    vector<int> points;
    long long last = LLONG_MIN;
    for (auto& iv : intervals) {
        if (iv.first > last) { last = iv.second; points.push_back(iv.second); }
    }
    return points;
}

int main() {
    vector<pair<int,int>> intervals = {{0,3},{2,6},{3,4},{6,9}};
    auto pts = minStabbingPoints(intervals);
    cout << "{";
    for (size_t i = 0; i < pts.size(); i++) cout << pts[i] << (i + 1 < pts.size() ? ", " : "");
    cout << "}\n";
    return 0;
}
