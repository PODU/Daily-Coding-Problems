// Merge overlapping intervals: sort by start, merge when next.start <= current.end.
// Time: O(n log n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<int,int>> iv = {{1,3},{5,8},{4,10},{20,25}};
    sort(iv.begin(), iv.end());
    vector<pair<int,int>> res;
    for (auto& p : iv) {
        if (!res.empty() && p.first <= res.back().second)
            res.back().second = max(res.back().second, p.second);
        else
            res.push_back(p);
    }
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << "(" << res[i].first << ", " << res[i].second << ")";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
