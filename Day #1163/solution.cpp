// Greedy: sort intervals by end ascending; keep if start >= last kept end; count removals. O(n log n) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

int eraseOverlapIntervals(vector<pair<int,int>>& intervals) {
    sort(intervals.begin(), intervals.end(), [](const pair<int,int>& a, const pair<int,int>& b) {
        return a.second < b.second;
    });
    int removals = 0;
    long long lastEnd = LLONG_MIN;
    for (auto& iv : intervals) {
        if (iv.first >= lastEnd) {
            lastEnd = iv.second;
        } else {
            removals++;
        }
    }
    return removals;
}

int main() {
    vector<pair<int,int>> intervals = {{7, 9}, {2, 4}, {5, 8}};
    cout << eraseOverlapIntervals(intervals) << "\n";
    return 0;
}
