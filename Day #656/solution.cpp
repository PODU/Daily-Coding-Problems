// Flood fill via BFS from start pixel; recolor only cells equal to original color.
// Guard against infinite loop when new color == original. Time O(rows*cols), space O(rows*cols).
#include <iostream>
#include <queue>
#include <vector>

void floodFill(std::vector<std::vector<char>>& img, int sr, int sc, char color) {
    int rows = img.size(), cols = img[0].size();
    char orig = img[sr][sc];
    if (orig == color) return;
    std::queue<std::pair<int, int>> q;
    q.push({sr, sc});
    img[sr][sc] = color;
    int dr[] = {-1, 1, 0, 0}, dc[] = {0, 0, -1, 1};
    while (!q.empty()) {
        int r = q.front().first, c = q.front().second;
        q.pop();
        for (int d = 0; d < 4; ++d) {
            int nr = r + dr[d], nc = c + dc[d];
            if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && img[nr][nc] == orig) {
                img[nr][nc] = color;
                q.push({nr, nc});
            }
        }
    }
}

int main() {
    std::vector<std::vector<char>> img = {
        {'B', 'B', 'W'},
        {'W', 'W', 'W'},
        {'W', 'W', 'W'},
        {'B', 'B', 'B'}};
    floodFill(img, 2, 2, 'G');
    for (auto& row : img) {
        for (size_t j = 0; j < row.size(); ++j) {
            std::cout << row[j];
            if (j + 1 < row.size()) std::cout << ' ';
        }
        std::cout << "\n";
    }
    return 0;
}
