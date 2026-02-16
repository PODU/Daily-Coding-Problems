// Flood fill via BFS from the seed pixel. Time O(R*C), Space O(R*C).
#include <bits/stdc++.h>
using namespace std;

void floodFill(vector<vector<char>>& img, int sr, int sc, char c) {
    char src = img[sr][sc];
    if (src == c) return;
    int R = img.size(), C = img[0].size();
    queue<pair<int,int>> q; q.push({sr, sc}); img[sr][sc] = c;
    int dx[] = {0,0,1,-1}, dy[] = {1,-1,0,0};
    while (!q.empty()) {
        auto [r, co] = q.front(); q.pop();
        for (int d = 0; d < 4; d++) {
            int nr = r + dx[d], nc = co + dy[d];
            if (nr >= 0 && nr < R && nc >= 0 && nc < C && img[nr][nc] == src) {
                img[nr][nc] = c; q.push({nr, nc});
            }
        }
    }
}

int main() {
    vector<vector<char>> img = {{'B','B','W'},{'W','W','W'},{'W','W','W'},{'B','B','B'}};
    floodFill(img, 2, 2, 'G');
    for (auto& row : img) {
        for (int i = 0; i < (int)row.size(); i++) cout << row[i] << (i + 1 < (int)row.size() ? " " : "");
        cout << "\n";
    }
}
