// Zigzag print: place each char at advancing column, row bounces 0..k-1..0.
// Build k row buffers, fill columns; rtrim each row. Time O(n*k), Space O(n*k).
#include <bits/stdc++.h>
using namespace std;

vector<string> zigzag(const string &s, int k) {
    int n = s.size();
    vector<string> grid(k, string(n, ' '));
    int row = 0, dir = 1;
    for (int col = 0; col < n; ++col) {
        grid[row][col] = s[col];
        if (k > 1) {
            if (row == 0) dir = 1;
            else if (row == k - 1) dir = -1;
            row += dir;
        }
    }
    for (auto &r : grid) {
        size_t end = r.find_last_not_of(' ');
        r = (end == string::npos) ? "" : r.substr(0, end + 1);
    }
    return grid;
}

int main() {
    for (auto &r : zigzag("thisisazigzag", 4)) cout << r << "\n";
    return 0;
}
