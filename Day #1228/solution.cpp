// Merge overlapping intervals: sort by start, sweep merging when start <= last end.
// Time: O(n log n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

vector<pair<int,int>> merge(vector<pair<int,int>> v) {
    sort(v.begin(), v.end());
    vector<pair<int,int>> res;
    for (auto& iv : v) {
        if (!res.empty() && iv.first <= res.back().second)
            res.back().second = max(res.back().second, iv.second);
        else
            res.push_back(iv);
    }
    return res;
}

int main() {
    vector<pair<int,int>> in = {{1,3},{5,8},{4,10},{20,25}};
    auto out = merge(in);
    cout << "[";
    for (size_t i = 0; i < out.size(); ++i) {
        cout << "(" << out[i].first << ", " << out[i].second << ")";
        if (i + 1 < out.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
