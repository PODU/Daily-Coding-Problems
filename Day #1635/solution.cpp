// Minimum stabbing points: greedy sort intervals by right endpoint; add right
// endpoint when current interval is unstabbed. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<int,int>> intervals = {{1,4},{4,5},{7,9},{9,12}};
    sort(intervals.begin(), intervals.end(),
         [](const pair<int,int>& a, const pair<int,int>& b){ return a.second < b.second; });
    vector<int> points;
    long long last = LLONG_MIN;
    for (auto& iv : intervals) {
        if (iv.first > last) { points.push_back(iv.second); last = iv.second; }
    }
    cout << "[";
    for (size_t i = 0; i < points.size(); ++i) {
        if (i) cout << ", ";
        cout << points[i];
    }
    cout << "]" << endl;
    return 0;
}
