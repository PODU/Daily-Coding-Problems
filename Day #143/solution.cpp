// Partition around pivot into <x, ==x, >x. Stable bucket collect to match expected order. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> partitionList(const vector<int>& lst, int x) {
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
    auto r = partitionList(lst, 10);
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) cout << r[i] << (i + 1 < r.size() ? ", " : "");
    cout << "]" << endl; // [9, 3, 5, 10, 10, 12, 14]
    return 0;
}
