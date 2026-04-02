// Day 1285: Print a string in zigzag form across k lines.
// Place char i at column i, row = triangle-wave of i. Time O(n*k) to render, Space O(n*k).
#include <bits/stdc++.h>
using namespace std;

void zigzag(const string& s, int k) {
    int n = s.size();
    if (k <= 1) { cout << s << "\n"; return; }
    int period = 2 * (k - 1);
    vector<string> grid(k, string(n, ' '));
    for (int i = 0; i < n; ++i) {
        int pos = i % period;
        int row = (pos < k) ? pos : period - pos;
        grid[row][i] = s[i];
    }
    for (auto& line : grid) {
        int end = line.find_last_not_of(' ');
        cout << (end == (int)string::npos ? "" : line.substr(0, end + 1)) << "\n";
    }
}

int main() {
    zigzag("thisisazigzag", 4);
    return 0;
}
