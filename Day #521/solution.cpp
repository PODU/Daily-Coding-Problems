// Zigzag: place char i at (zigzag-row, column i) in a k x n grid; print rows. O(n*k).
#include <bits/stdc++.h>
using namespace std;

void zigzag(const string& s, int k) {
    int n = s.size();
    vector<string> grid(k, string(n, ' '));
    int row = 0, dir = (k == 1 ? 0 : 1);
    for (int i = 0; i < n; i++) {
        grid[row][i] = s[i];
        if (row == 0) dir = 1;
        else if (row == k - 1) dir = -1;
        row += dir;
    }
    for (auto& r : grid) {
        int end = r.find_last_not_of(' ');
        cout << (end == (int)string::npos ? "" : r.substr(0, end + 1)) << "\n";
    }
}

int main() {
    zigzag("thisisazigzag", 4);
    return 0;
}
