// Min intervals to remove for non-overlapping (touching allowed).
// Greedy: sort by end, keep intervals whose start >= last kept end. O(n log n) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

int minRemovals(vector<pair<int,int>> intervals) {
    sort(intervals.begin(), intervals.end(),
         [](const pair<int,int>& a, const pair<int,int>& b){ return a.second < b.second; });
    int kept = 0, lastEnd = INT_MIN;
    for (auto& iv : intervals) {
        if (iv.first >= lastEnd) { kept++; lastEnd = iv.second; }
    }
    return (int)intervals.size() - kept;
}

int main() {
    vector<pair<int,int>> intervals = {{7,9},{2,4},{5,8}};
    cout << minRemovals(intervals) << "\n";
    return 0;
}
