// Power set via bitmask, sorted by (size, lexicographic) to match example order.
// Time O(n*2^n), Space O(2^n).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> s = {1, 2, 3};
    int n = s.size();
    vector<vector<int>> subsets;
    for (int mask = 0; mask < (1 << n); mask++) {
        vector<int> sub;
        for (int i = 0; i < n; i++)
            if (mask & (1 << i)) sub.push_back(s[i]);
        subsets.push_back(sub);
    }
    sort(subsets.begin(), subsets.end(), [](const vector<int>& a, const vector<int>& b) {
        if (a.size() != b.size()) return a.size() < b.size();
        return a < b;
    });
    string out = "{";
    for (size_t k = 0; k < subsets.size(); k++) {
        if (k) out += ", ";
        out += "{";
        for (size_t i = 0; i < subsets[k].size(); i++) {
            if (i) out += ", ";
            out += to_string(subsets[k][i]);
        }
        out += "}";
    }
    out += "}";
    cout << out << "\n";
    return 0;
}
