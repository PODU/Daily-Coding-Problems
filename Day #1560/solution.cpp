// Greedy interval stabbing: sort by right endpoint; for the smallest uncovered right,
// pick point = max left among intervals containing it, cover them all. Time O(n^2), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> stabPoints(vector<pair<int,int>> iv) {
    sort(iv.begin(), iv.end(), [](auto& a, auto& b){ return a.second < b.second; });
    int n = iv.size();
    vector<bool> covered(n, false);
    vector<int> points;
    for (int i = 0; i < n; i++) {
        if (covered[i]) continue;
        int r = iv[i].second;
        int point = INT_MIN;
        for (int j = i; j < n; j++)
            if (!covered[j] && iv[j].first <= r)
                point = max(point, iv[j].first);
        points.push_back(point);
        for (int j = i; j < n; j++)
            if (!covered[j] && iv[j].first <= point && point <= iv[j].second)
                covered[j] = true;
    }
    return points;
}

int main() {
    vector<pair<int,int>> iv = {{0,3},{2,6},{3,4},{6,9}};
    vector<int> pts = stabPoints(iv);
    cout << "{";
    for (size_t i = 0; i < pts.size(); i++) {
        if (i) cout << ", ";
        cout << pts[i];
    }
    cout << "}\n";
    return 0;
}
