// Day 839: Max number of dictionary words packed on an NxN board.
// For each word collect all valid adjacent-path placements (DFS), then backtrack
// over words choosing a disjoint set to maximize the count.
// Time O(exponential worst-case) on small boards; placement search bounded by board size.
#include <bits/stdc++.h>
using namespace std;

int R, C;

// Each placement encoded as a bitmask over R*C tiles.
void dfs(const vector<string>& mat, const string& word, int r, int c, int idx,
         long long used, set<long long>& placements) {
    if (mat[r][c] != word[idx]) return;
    int bit = r * C + c;
    used |= (1LL << bit);
    if (idx == (int)word.size() - 1) {
        placements.insert(used);
        return;
    }
    int dr[] = {1, -1, 0, 0}, dc[] = {0, 0, 1, -1};
    for (int d = 0; d < 4; ++d) {
        int nr = r + dr[d], nc = c + dc[d];
        if (nr >= 0 && nr < R && nc >= 0 && nc < C && !(used & (1LL << (nr * C + nc))))
            dfs(mat, word, nr, nc, idx + 1, used, placements);
    }
}

vector<long long> findPlacements(const vector<string>& mat, const string& word) {
    set<long long> placements;
    for (int i = 0; i < R; ++i)
        for (int j = 0; j < C; ++j)
            dfs(mat, word, i, j, 0, 0, placements);
    return vector<long long>(placements.begin(), placements.end());
}

int best;
vector<vector<long long>> wordPlacements;

void backtrack(int i, long long occupied, int count) {
    best = max(best, count);
    if (i == (int)wordPlacements.size()) return;
    backtrack(i + 1, occupied, count); // skip
    for (long long tiles : wordPlacements[i])
        if (!(occupied & tiles))
            backtrack(i + 1, occupied | tiles, count + 1);
}

int maxWords(const vector<string>& mat, const vector<string>& dict) {
    wordPlacements.clear();
    for (const string& w : dict) {
        auto p = findPlacements(mat, w);
        if (!p.empty()) wordPlacements.push_back(p);
    }
    best = 0;
    backtrack(0, 0, 0);
    return best;
}

int main() {
    vector<string> mat = {"ean", "tti", "ara"};
    R = mat.size();
    C = mat[0].size();
    vector<string> dict = {"eat", "rain", "in", "rat"};
    cout << maxWords(mat, dict) << endl;
    return 0;
}
