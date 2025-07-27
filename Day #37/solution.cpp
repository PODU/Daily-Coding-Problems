// Power set via bitmask 0..2^n-1, then order subsets by (size, element order).
// O(2^n * n) time, O(2^n * n) space.
#include <iostream>
#include <vector>
#include <algorithm>
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
    for (size_t i = 0; i < subsets.size(); i++) {
        if (i) out += ", ";
        out += "{";
        for (size_t j = 0; j < subsets[i].size(); j++) {
            if (j) out += ", ";
            out += to_string(subsets[i][j]);
        }
        out += "}";
    }
    out += "}";
    cout << out << endl;
    return 0;
}
