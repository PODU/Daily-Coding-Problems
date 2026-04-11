// Day 1342: Pack the maximum number of dictionary words onto a letter grid (vertex-disjoint adjacency paths).
// Enumerate each word's placements (cell bitmasks) via DFS, then backtrack to select max disjoint set. Exponential worst case.
#include <bits/stdc++.h>
using namespace std;

int N, M;
vector<string> grid;
int dr[4] = {-1, 1, 0, 0}, dc[4] = {0, 0, -1, 1};

void dfsWord(const string& w, int idx, int r, int c, unsigned long long mask, vector<unsigned long long>& out) {
    mask |= (1ULL << (r * M + c));
    if (idx == (int)w.size() - 1) { out.push_back(mask); return; }
    for (int k = 0; k < 4; k++) {
        int nr = r + dr[k], nc = c + dc[k];
        if (nr < 0 || nr >= N || nc < 0 || nc >= M) continue;
        if (mask & (1ULL << (nr * M + nc))) continue;
        if (grid[nr][nc] != w[idx + 1]) continue;
        dfsWord(w, idx + 1, nr, nc, mask, out);
    }
}

vector<vector<unsigned long long>> placements;
int best;

void backtrack(int i, unsigned long long used, int count) {
    if (count + (int)(placements.size() - i) <= best) return;
    if (i == (int)placements.size()) { best = max(best, count); return; }
    backtrack(i + 1, used, count); // skip this word
    for (unsigned long long m : placements[i])
        if (!(m & used)) backtrack(i + 1, used | m, count + 1);
}

int maxWords(vector<string>& dict, vector<string>& board) {
    grid = board; N = board.size(); M = board[0].size();
    placements.clear();
    for (auto& w : dict) {
        vector<unsigned long long> masks;
        for (int r = 0; r < N; r++)
            for (int c = 0; c < M; c++)
                if (grid[r][c] == w[0]) dfsWord(w, 0, r, c, 0, masks);
        sort(masks.begin(), masks.end());
        masks.erase(unique(masks.begin(), masks.end()), masks.end());
        if (!masks.empty()) placements.push_back(masks);
    }
    best = 0;
    backtrack(0, 0, 0);
    return best;
}

int main() {
    vector<string> dict = {"eat", "rain", "in", "rat"};
    vector<string> board = {"ean", "tti", "ara"};
    cout << maxWords(dict, board) << "\n";
    return 0;
}
