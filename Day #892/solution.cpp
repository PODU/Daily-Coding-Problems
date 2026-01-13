// Power set via bitmask iteration over 2^n subsets, then sorted by (size, elements).
// Time: O(n*2^n), Space: O(n*2^n) to hold all subsets.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> s = {1, 2, 3};
    int n = s.size();
    vector<vector<int>> subsets;
    for (int mask = 0; mask < (1 << n); ++mask) {
        vector<int> cur;
        for (int i = 0; i < n; ++i)
            if (mask & (1 << i)) cur.push_back(s[i]);
        subsets.push_back(cur);
    }
    sort(subsets.begin(), subsets.end(), [](const vector<int>& a, const vector<int>& b) {
        if (a.size() != b.size()) return a.size() < b.size();
        return a < b;
    });
    cout << "{";
    for (size_t i = 0; i < subsets.size(); ++i) {
        if (i) cout << ", ";
        cout << "{";
        for (size_t j = 0; j < subsets[i].size(); ++j) {
            if (j) cout << ", ";
            cout << subsets[i][j];
        }
        cout << "}";
    }
    cout << "}" << endl;
    return 0;
}
