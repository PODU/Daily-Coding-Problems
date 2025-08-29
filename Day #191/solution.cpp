// Day 191: Min intervals to remove = n - max non-overlapping set (touching allowed).
// Greedy by earliest end. Time O(n log n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minRemovals(vector<pair<int,int>> iv) {
    sort(iv.begin(), iv.end(), [](auto& a, auto& b){ return a.second < b.second; });
    int kept = 0, end = INT_MIN;
    for (auto& p : iv)
        if (p.first >= end) { kept++; end = p.second; }
    return (int)iv.size() - kept;
}

int main() {
    cout << minRemovals({{7, 9}, {2, 4}, {5, 8}}) << "\n";
    return 0;
}
