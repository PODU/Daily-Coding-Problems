// Day 151: Flood fill via BFS. Replace target pixel's connected same-colored
// region with new color. Time O(R*C), Space O(R*C).
#include <bits/stdc++.h>
using namespace std;

void floodFill(vector<vector<char>>& m, int r, int c, char color) {
    int R = m.size(), C = m[0].size();
    char target = m[r][c];
    if (target == color) return;
    queue<pair<int,int>> q;
    q.push(make_pair(r, c));
    m[r][c] = color;
    int dr[] = {-1,1,0,0}, dc[] = {0,0,-1,1};
    while (!q.empty()) {
        int x = q.front().first, y = q.front().second; q.pop();
        for (int d = 0; d < 4; d++) {
            int nx = x+dr[d], ny = y+dc[d];
            if (nx>=0 && nx<R && ny>=0 && ny<C && m[nx][ny]==target) {
                m[nx][ny] = color;
                q.push(make_pair(nx, ny));
            }
        }
    }
}

int main() {
    vector<vector<char>> m = {
        {'B','B','W'},
        {'W','W','W'},
        {'W','W','W'},
        {'B','B','B'}
    };
    floodFill(m, 2, 2, 'G');
    for (auto& row : m) {
        for (int j = 0; j < (int)row.size(); j++)
            cout << row[j] << (j+1<(int)row.size() ? " " : "");
        cout << "\n";
    }
    return 0;
}
