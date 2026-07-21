// Day 1851: Zigzag string across k lines.
// Place char i at row=zigzag(i), col=i in a grid; print rows. O(n*k) build, O(n) chars. Space O(n*k).
#include <bits/stdc++.h>
using namespace std;

vector<string> zigzag(const string& s, int k) {
    int n = (int)s.size();
    vector<string> grid(k, string(n, ' '));
    int period = (k <= 1) ? 1 : 2 * (k - 1);
    for (int i = 0; i < n; i++) {
        int pos = (k <= 1) ? 0 : i % period;
        int row = (pos < k) ? pos : period - pos;
        grid[row][i] = s[i];
    }
    // trim trailing spaces
    for (auto& r : grid) {
        size_t e = r.find_last_not_of(' ');
        r = (e == string::npos) ? string() : r.substr(0, e + 1);
    }
    return grid;
}

int main() {
    string s = "thisisazigzag";
    int k = 4;
    for (auto& line : zigzag(s, k)) cout << line << "\n";
    return 0;
}
