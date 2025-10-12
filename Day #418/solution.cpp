// Day 418: Two-pass greedy. Each gets >= 1; more than any lower neighbor. Like candy distribution.
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<int> bonuses(const vector<int>& lines) {
    int n = lines.size();
    vector<int> res(n, 1);
    for (int i = 1; i < n; ++i)
        if (lines[i] > lines[i-1]) res[i] = res[i-1] + 1;
    for (int i = n - 2; i >= 0; --i)
        if (lines[i] > lines[i+1]) res[i] = max(res[i], res[i+1] + 1);
    return res;
}

int main() {
    vector<int> lines = {10, 40, 200, 1000, 60, 30};
    vector<int> res = bonuses(lines);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        if (i) cout << ", ";
        cout << res[i];
    }
    cout << "]\n";
    return 0;
}
