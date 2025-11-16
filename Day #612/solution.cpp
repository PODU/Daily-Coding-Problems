// Day 612: Min intervals to remove so the rest are non-overlapping (touching allowed).
// Approach: sort by end, greedily keep max non-overlapping; answer = total - kept. Time O(n log n).
#include <bits/stdc++.h>
using namespace std;

int minRemovals(vector<pair<int,int>> intervals) {
    sort(intervals.begin(), intervals.end(),
         [](const auto& a, const auto& b) { return a.second < b.second; });
    int kept = 0, end = INT_MIN;
    for (auto& iv : intervals)
        if (iv.first >= end) { kept++; end = iv.second; }
    return (int)intervals.size() - kept;
}

int main() {
    vector<pair<int,int>> intervals = {{7, 9}, {2, 4}, {5, 8}};
    cout << minRemovals(intervals) << "\n"; // 1
    return 0;
}
