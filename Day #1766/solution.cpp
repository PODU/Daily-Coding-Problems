// Merge overlapping intervals: sort by start, then sweep merging when the next
// start <= current end. Time: O(n log n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

vector<pair<int,int>> merge(vector<pair<int,int>> iv) {
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
    vector<pair<int,int>> iv = {{1,3},{5,8},{4,10},{20,25}};
    auto r = merge(iv);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) {
        cout << "(" << r[i].first << ", " << r[i].second << ")";
        if (i + 1 < r.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
