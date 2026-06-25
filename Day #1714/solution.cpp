// Max words packed: boggle DFS to enumerate placements + backtracking over words. Exponential worst case, small dict.
#include <bits/stdc++.h>
using namespace std;

int n;
vector<string> board;
vector<vector<bool>> taken;
int DR[4] = {-1, 1, 0, 0};
int DC[4] = {0, 0, -1, 1};

void dfs(int r, int c, const string& w, int idx, set<int>& path, set<set<int>>& found) {
    if (r < 0 || r >= n || c < 0 || c >= n) return;
    if (taken[r][c] || path.count(r * n + c) || board[r][c] != w[idx]) return;
    path.insert(r * n + c);
    if (idx == (int)w.size() - 1) {
        found.insert(path);
    } else {
        for (int d = 0; d < 4; d++)
            dfs(r + DR[d], c + DC[d], w, idx + 1, path, found);
    }
    path.erase(r * n + c);
}

vector<set<int>> findPlacements(const string& w) {
    set<set<int>> found;
    for (int i = 0; i < n; i++)
        for (int j = 0; j < n; j++) {
            set<int> path;
            dfs(i, j, w, 0, path, found);
        }
    return vector<set<int>>(found.begin(), found.end());
}

int best = 0;
vector<string> words;
set<string> used;

void search() {
    best = max(best, (int)used.size());
    for (const string& w : words) {
        if (used.count(w)) continue;
        for (auto& placement : findPlacements(w)) {
            for (int cell : placement) taken[cell / n][cell % n] = true;
            used.insert(w);
            search();
            used.erase(w);
            for (int cell : placement) taken[cell / n][cell % n] = false;
        }
    }
}

int main() {
    board = {"ean", "tti", "ara"};
    n = 3;
    taken.assign(n, vector<bool>(n, false));
    words = {"eat", "rain", "in", "rat"};
    search();
    cout << best << "\n";
    return 0;
}
