// K nearest points: max-heap of size k by squared Euclidean distance,
// then sort the k results by (dist, original index) for deterministic output.
// Time: O(n log k), Space: O(k).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<long long, long long>> points = {{0, 0}, {5, 4}, {3, 1}};
    long long cx = 1, cy = 2;
    int k = 2;

    auto dist2 = [&](const pair<long long, long long>& p) {
        long long dx = p.first - cx, dy = p.second - cy;
        return dx * dx + dy * dy;
    };

    // max-heap of (dist2, index) keeping k smallest
    priority_queue<pair<long long, int>> heap;
    for (int i = 0; i < (int)points.size(); ++i) {
        heap.push({dist2(points[i]), i});
        if ((int)heap.size() > k) heap.pop();
    }

    vector<int> idx;
    while (!heap.empty()) { idx.push_back(heap.top().second); heap.pop(); }
    sort(idx.begin(), idx.end(), [&](int a, int b) {
        long long da = dist2(points[a]), db = dist2(points[b]);
        if (da != db) return da < db;
        return a < b;
    });

    cout << "[";
    for (size_t i = 0; i < idx.size(); ++i) {
        if (i) cout << ", ";
        cout << "(" << points[idx[i]].first << ", " << points[idx[i]].second << ")";
    }
    cout << "]" << endl;
    return 0;
}
