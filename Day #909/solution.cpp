// Minimum points covering all closed intervals: greedy, sort by END ascending; open a new
// group when start>anchor-end, pick each group's MAX start as its point. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> minPoints(vector<pair<int,int>> intervals) {
    sort(intervals.begin(), intervals.end(),
         [](auto& a, auto& b){ return a.second < b.second; });
    vector<int> points;
    bool have = false;
    long long anchorEnd = LLONG_MIN, groupMaxStart = LLONG_MIN;
    for (auto& iv : intervals) {
        if (!have || iv.first > anchorEnd) {
            if (have) points.push_back((int)groupMaxStart);
            have = true; anchorEnd = iv.second; groupMaxStart = iv.first;
        } else if (iv.first > groupMaxStart) {
            groupMaxStart = iv.first;
        }
    }
    if (have) points.push_back((int)groupMaxStart);
    return points;
}

int main() {
    vector<pair<int,int>> intervals = {{0,3},{2,6},{3,4},{6,9}};
    vector<int> pts = minPoints(intervals);
    cout << "{";
    for (size_t i = 0; i < pts.size(); i++)
        cout << pts[i] << (i + 1 < pts.size() ? ", " : "");
    cout << "}" << endl;
    return 0;
}
