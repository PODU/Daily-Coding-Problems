// Day 840: Print a string in zigzag form across k lines.
// Char i sits at column i; its row follows the zigzag 0,1,..,k-1,k-2,..,1,0,...
// Build k rows of spaces, place each char, print with trailing spaces trimmed. Time O(N*k).
#include <bits/stdc++.h>
using namespace std;

string zigzag(const string& s, int k) {
    if (k <= 0) return "";
    if (k == 1) return s;
    int n = s.size();
    vector<string> rows(k, string(n, ' '));
    int row = 0, step = 1;
    for (int i = 0; i < n; ++i) {
        rows[row][i] = s[i];
        if (row == 0) step = 1;
        else if (row == k - 1) step = -1;
        row += step;
    }
    string out;
    for (int r = 0; r < k; ++r) {
        string line = rows[r];
        size_t end = line.find_last_not_of(' ');
        line = (end == string::npos) ? "" : line.substr(0, end + 1);
        out += line;
        if (r != k - 1) out += "\n";
    }
    return out;
}

int main() {
    cout << zigzag("thisisazigzag", 4) << endl;
    return 0;
}
