// Day 1113 - Validate an American-style crossword grid ('#' black, '.' white)
// Approach: 180-degree symmetry, every white cell in across & down run >= 3,
// and white connectivity. Time: O(N^2), Space: O(N^2).
#include <bits/stdc++.h>
using namespace std;

bool isCrossword(const vector<string>& grid) {
    int n = grid.size();
    if (n == 0) return false;
    for (auto& r : grid) if ((int)r.size() != n) return false;

    for (int i = 0; i < n; ++i)
        for (int j = 0; j < n; ++j)
            if ((grid[i][j] == '#') != (grid[n-1-i][n-1-j] == '#')) return false;

    for (int i = 0; i < n; ++i)
        for (int j = 0; j < n; ++j)
            if (grid[i][j] == '.') {
                int len = 1, k = j - 1;
                while (k >= 0 && grid[i][k] == '.') { ++len; --k; }
                k = j + 1;
                while (k < n && grid[i][k] == '.') { ++len; ++k; }
                if (len < 3) return false;
                len = 1; k = i - 1;
                while (k >= 0 && grid[k][j] == '.') { ++len; --k; }
                k = i + 1;
                while (k < n && grid[k][j] == '.') { ++len; ++k; }
                if (len < 3) return false;
            }

    vector<pair<int,int>> whites;
    for (int i = 0; i < n; ++i)
        for (int j = 0; j < n; ++j)
            if (grid[i][j] == '.') whites.push_back({i, j});
    if (whites.empty()) return true;

    vector<vector<bool>> seen(n, vector<bool>(n, false));
    vector<pair<int,int>> st = {whites[0]};
    seen[whites[0].first][whites[0].second] = true;
    int cnt = 1;
    int dx[] = {1,-1,0,0}, dy[] = {0,0,1,-1};
    while (!st.empty()) {
        auto [i, j] = st.back(); st.pop_back();
        for (int d = 0; d < 4; ++d) {
            int ni = i + dx[d], nj = j + dy[d];
            if (ni>=0&&ni<n&&nj>=0&&nj<n&&grid[ni][nj]=='.'&&!seen[ni][nj]) {
                seen[ni][nj] = true; ++cnt; st.push_back({ni, nj});
            }
        }
    }
    return cnt == (int)whites.size();
}

int main() {
    vector<string> valid = {"...##", ".....", ".....", ".....", "##..."};
    vector<string> invalid = {".....", ".....", ".....", ".....", "....#"};
    cout << "Grid 1 valid: " << (isCrossword(valid) ? "True" : "False") << endl;
    cout << "Grid 2 valid: " << (isCrossword(invalid) ? "True" : "False") << endl;
    return 0;
}
