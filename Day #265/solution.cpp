// Day 265: Minimum bonuses. Two-pass scan (left-to-right then right-to-left),
// each worker gets max of the two passes. Time O(n), space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> bonuses(const vector<int>& lines) {
    int n = lines.size();
    vector<int> b(n, 1);
    for (int i = 1; i < n; ++i)
        if (lines[i] > lines[i - 1]) b[i] = b[i - 1] + 1;
    for (int i = n - 2; i >= 0; --i)
        if (lines[i] > lines[i + 1]) b[i] = max(b[i], b[i + 1] + 1);
    return b;
}

int main() {
    vector<int> lines = {10, 40, 200, 1000, 60, 30};
    vector<int> b = bonuses(lines);
    cout << "[";
    for (size_t i = 0; i < b.size(); ++i)
        cout << b[i] << (i + 1 < b.size() ? ", " : "");
    cout << "]" << endl;
    return 0;
}
