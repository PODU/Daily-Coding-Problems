// Greedy: sort intervals by end; keep interval if start >= last kept end.
// Answer = total - kept. Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

int eraseOverlapIntervals(vector<pair<int,int>> intervals) {
    sort(intervals.begin(), intervals.end(),
         [](const pair<int,int>& a, const pair<int,int>& b){ return a.second < b.second; });
    int kept = 0, lastEnd = INT_MIN;
    for (auto& it : intervals) {
        if (it.first >= lastEnd) { kept++; lastEnd = it.second; }
    }
    return (int)intervals.size() - kept;
}

int main() {
    vector<pair<int,int>> intervals = {{7,9},{2,4},{5,8}};
    cout << eraseOverlapIntervals(intervals) << endl;
    return 0;
}
