// Day 200: Minimum points stabbing all intervals.
// Greedy: sort by right endpoint; pick the right end whenever current interval is unstabbed.
// Time: O(n log n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

vector<int> stab(vector<pair<int,int>> iv) {
    sort(iv.begin(), iv.end(), [](auto& a, auto& b){ return a.second < b.second; });
    vector<int> pts;
    long long last = LLONG_MIN;
    for (auto& p : iv) {
        if (p.first > last) { last = p.second; pts.push_back((int)last); }
    }
    return pts;
}

int main() {
    auto r = stab({{1, 4}, {4, 5}, {7, 9}, {9, 12}});
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]" << endl; // [4, 9]
    return 0;
}
