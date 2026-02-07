// Day 1035: Smallest bonuses so each employee beats any lower-output neighbor.
// Two-pass greedy: left-to-right then right-to-left taking max. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> bonuses(const vector<int>& a) {
    int n = a.size();
    vector<int> b(n, 1);
    for (int i = 1; i < n; ++i)
        if (a[i] > a[i - 1]) b[i] = b[i - 1] + 1;
    for (int i = n - 2; i >= 0; --i)
        if (a[i] > a[i + 1]) b[i] = max(b[i], b[i + 1] + 1);
    return b;
}

int main() {
    vector<int> a = {10, 40, 200, 1000, 60, 30};
    vector<int> b = bonuses(a);
    cout << "[";
    for (size_t i = 0; i < b.size(); ++i) cout << b[i] << (i + 1 < b.size() ? ", " : "");
    cout << "]\n";
}
