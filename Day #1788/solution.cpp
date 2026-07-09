// k nearest points: stable sort by squared Euclidean distance, take first k.
// Time O(N log N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<int,int>> pts = {{0,0},{5,4},{3,1}};
    int cx = 1, cy = 2, k = 2;
    vector<int> idx(pts.size());
    iota(idx.begin(), idx.end(), 0);
    stable_sort(idx.begin(), idx.end(), [&](int a, int b){
        long da = (long)(pts[a].first-cx)*(pts[a].first-cx)+(long)(pts[a].second-cy)*(pts[a].second-cy);
        long db = (long)(pts[b].first-cx)*(pts[b].first-cx)+(long)(pts[b].second-cy)*(pts[b].second-cy);
        return da < db;
    });
    cout << "[";
    for (int i = 0; i < k; i++) {
        if (i) cout << ", ";
        cout << "(" << pts[idx[i]].first << ", " << pts[idx[i]].second << ")";
    }
    cout << "]\n";
    return 0;
}
