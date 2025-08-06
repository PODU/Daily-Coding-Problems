// Day 77: Merge overlapping intervals. Sort by start, then sweep merging.
// Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<pair<int,int>> mergeIntervals(vector<pair<int,int>> iv) {
    sort(iv.begin(), iv.end());
    vector<pair<int,int>> res;
    for (auto& p : iv) {
        if (!res.empty() && p.first <= res.back().second)
            res.back().second = max(res.back().second, p.second);
        else
            res.push_back(p);
    }
    return res;
}

int main() {
    vector<pair<int,int>> in = {{1,3},{5,8},{4,10},{20,25}};
    auto res = mergeIntervals(in);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << "(" << res[i].first << ", " << res[i].second << ")";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n"; // [(1, 3), (4, 10), (20, 25)]
    return 0;
}
