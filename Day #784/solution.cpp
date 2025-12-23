// Word search L-to-R / U-to-D only: scan each row and column for target substring.
// Time O(R*C*L), Space O(max(R,C)).
#include <bits/stdc++.h>
using namespace std;

bool findWord(const vector<vector<char>>& m, const string& target) {
    int R = m.size(), C = R ? m[0].size() : 0;
    for (int r = 0; r < R; r++) {
        string row;
        for (int c = 0; c < C; c++) row += m[r][c];
        if (row.find(target) != string::npos) return true;
    }
    for (int c = 0; c < C; c++) {
        string col;
        for (int r = 0; r < R; r++) col += m[r][c];
        if (col.find(target) != string::npos) return true;
    }
    return false;
}

int main() {
    vector<vector<char>> matrix = {
        {'F','A','C','I'},
        {'O','B','Q','P'},
        {'A','N','O','B'},
        {'M','A','S','S'}
    };
    cout << (findWord(matrix, "FOAM") ? "true" : "false") << "\n";
    return 0;
}
