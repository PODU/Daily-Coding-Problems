// Day 1479: Partition a list into <x, ==x, >x around pivot x.
// Stable bucketing into three lists preserves relative order. Time O(N), Space O(N).
#include <bits/stdc++.h>
using namespace std;

vector<int> partitionThree(const vector<int>& lst, int x) {
    vector<int> less, equal, greater;
    for (int v : lst) {
        if (v < x) less.push_back(v);
        else if (v == x) equal.push_back(v);
        else greater.push_back(v);
    }
    vector<int> res;
    res.insert(res.end(), less.begin(), less.end());
    res.insert(res.end(), equal.begin(), equal.end());
    res.insert(res.end(), greater.begin(), greater.end());
    return res;
}

int main() {
    auto r = partitionThree({9, 12, 3, 5, 14, 10, 10}, 10);
    cout << "[";
    for (size_t i = 0; i < r.size(); ++i) cout << (i ? ", " : "") << r[i];
    cout << "]\n";  // [9, 3, 5, 10, 10, 12, 14]
}
