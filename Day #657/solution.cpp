// Power set via bitmasks; sort subsets by size then numeric order. Time O(n*2^n), Space O(2^n).
#include <bits/stdc++.h>
using namespace std;
int main() {
    vector<int> s = {1, 2, 3};
    int n = s.size();
    vector<vector<int>> subsets;
    for (int mask = 0; mask < (1 << n); ++mask) {
        vector<int> sub;
        for (int i = 0; i < n; ++i)
            if (mask & (1 << i)) sub.push_back(s[i]);
        subsets.push_back(sub);
    }
    stable_sort(subsets.begin(), subsets.end(), [](const vector<int>& a, const vector<int>& b) {
        if (a.size() != b.size()) return a.size() < b.size();
        return a < b;
    });
    string out = "{";
    for (size_t i = 0; i < subsets.size(); ++i) {
        out += "{";
        for (size_t j = 0; j < subsets[i].size(); ++j) {
            out += to_string(subsets[i][j]);
            if (j + 1 < subsets[i].size()) out += ", ";
        }
        out += "}";
        if (i + 1 < subsets.size()) out += ", ";
    }
    out += "}";
    cout << out << endl;
    return 0;
}
