// K nearest points: max-heap of size k keyed by squared distance. Time O(n log k), Space O(k).
#include <iostream>
#include <vector>
#include <queue>
#include <algorithm>
using namespace std;

struct Pt { int x, y; };

vector<Pt> kNearest(vector<Pt>& pts, Pt c, int k) {
    // max-heap: (dist2, original index)
    priority_queue<pair<long long, int>> heap;
    for (int i = 0; i < (int)pts.size(); i++) {
        long long dx = pts[i].x - c.x, dy = pts[i].y - c.y;
        long long d2 = dx * dx + dy * dy;
        heap.push({d2, i});
        if ((int)heap.size() > k) heap.pop();
    }
    vector<int> idx;
    while (!heap.empty()) { idx.push_back(heap.top().second); heap.pop(); }
    sort(idx.begin(), idx.end()); // keep original order for stable output
    vector<Pt> res;
    for (int i : idx) res.push_back(pts[i]);
    return res;
}

int main() {
    vector<Pt> pts = {{0, 0}, {5, 4}, {3, 1}};
    Pt c = {1, 2};
    int k = 2;
    vector<Pt> res = kNearest(pts, c, k);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        cout << "(" << res[i].x << ", " << res[i].y << ")";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
