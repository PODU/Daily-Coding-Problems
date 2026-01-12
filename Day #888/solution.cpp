// Nearest k points: max-heap of size k by squared distance. Time O(n log k), Space O(k).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<int,int>> points = {{0,0},{5,4},{3,1}};
    pair<int,int> central = {1,2};
    int k = 2;

    // max-heap keyed by squared distance: {dist2, point}
    priority_queue<pair<long long, pair<int,int>>> heap;
    for (auto& p : points) {
        long long dx = p.first - central.first, dy = p.second - central.second;
        long long d2 = dx*dx + dy*dy;
        heap.push({d2, p});
        if ((int)heap.size() > k) heap.pop();
    }
    vector<pair<int,int>> res;
    while (!heap.empty()) { res.push_back(heap.top().second); heap.pop(); }
    sort(res.begin(), res.end());

    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        if (i) cout << ", ";
        cout << "(" << res[i].first << ", " << res[i].second << ")";
    }
    cout << "]" << endl;
    return 0;
}
