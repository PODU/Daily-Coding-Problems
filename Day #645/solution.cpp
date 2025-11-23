// Day 645: Find a word in a grid going left-to-right or top-to-bottom.
// Approach: scan every row and every column for the target as a substring start.
// Time: O(R*C*L), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

bool findWord(const vector<vector<char>>& g, const string& word) {
    int R = g.size(), C = g[0].size(), L = word.size();
    // horizontal
    for (int r = 0; r < R; r++)
        for (int c = 0; c + L <= C; c++) {
            bool ok = true;
            for (int k = 0; k < L; k++) if (g[r][c+k] != word[k]) { ok = false; break; }
            if (ok) return true;
        }
    // vertical
    for (int c = 0; c < C; c++)
        for (int r = 0; r + L <= R; r++) {
            bool ok = true;
            for (int k = 0; k < L; k++) if (g[r+k][c] != word[k]) { ok = false; break; }
            if (ok) return true;
        }
    return false;
}

int main() {
    vector<vector<char>> g = {
        {'F','A','C','I'},
        {'O','B','Q','P'},
        {'A','N','O','B'},
        {'M','A','S','S'}
    };
    cout << boolalpha << findWord(g, "FOAM") << "\n"; // true
    cout << boolalpha << findWord(g, "MASS") << "\n"; // true
    return 0;
}
