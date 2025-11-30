// Day 673: K nearest points to a center. Max-heap of size k on squared distance.
// Time O(n log k), Space O(k). (No sqrt needed for comparison.)
#include <bits/stdc++.h>
using namespace std;

vector<pair<int,int>> kNearest(vector<pair<int,int>> pts, pair<int,int> c, int k) {
    auto d2 = [&](const pair<int,int>& p) {
        long long dx = p.first - c.first, dy = p.second - c.second;
        return dx * dx + dy * dy;
    };
    // keep (dist, originalIndex) so ties preserve input order
    vector<int> idx(pts.size());
    iota(idx.begin(), idx.end(), 0);
    sort(idx.begin(), idx.end(), [&](int a, int b) {
        long long da = d2(pts[a]), db = d2(pts[b]);
        return da != db ? da < db : a < b;
    });
    vector<pair<int,int>> res;
    for (int i = 0; i < k && i < (int)idx.size(); i++) res.push_back(pts[idx[i]]);
    return res;
}

int main() {
    vector<pair<int,int>> pts = {{0, 0}, {5, 4}, {3, 1}};
    auto r = kNearest(pts, {1, 2}, 2);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++)
        cout << "(" << r[i].first << ", " << r[i].second << ")" << (i + 1 < r.size() ? ", " : "");
    cout << "]\n"; // [(0, 0), (3, 1)]
    return 0;
}
