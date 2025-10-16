// Day 441: Three-way partition around pivot x into <x, ==x, >x buckets.
// O(n) time, O(n) space (stable bucket order matches the example).
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
    vector<int> lst = {9, 12, 3, 5, 14, 10, 10};
    auto res = partitionThree(lst, 10);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n"; // [9, 3, 5, 10, 10, 12, 14]
    return 0;
}
