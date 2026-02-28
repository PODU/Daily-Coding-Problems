// Power set via bitmask enumeration, sorted by (size, lexicographic). O(2^n * n).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> nums = {1, 2, 3};
    int n = nums.size();
    vector<vector<int>> subsets;
    for (int mask = 0; mask < (1 << n); mask++) {
        vector<int> s;
        for (int i = 0; i < n; i++)
            if (mask & (1 << i)) s.push_back(nums[i]);
        subsets.push_back(s);
    }
    sort(subsets.begin(), subsets.end(), [](const vector<int>& a, const vector<int>& b) {
        if (a.size() != b.size()) return a.size() < b.size();
        return a < b;
    });
    string out = "[";
    for (size_t k = 0; k < subsets.size(); k++) {
        if (k) out += ", ";
        out += "[";
        for (size_t i = 0; i < subsets[k].size(); i++) {
            if (i) out += ", ";
            out += to_string(subsets[k][i]);
        }
        out += "]";
    }
    out += "]";
    cout << out << endl;
}
