// Island perimeter: +4 per land cell, -2 per adjacent right/down land pair. O(R*C) time, O(1) space.
#include <vector>
#include <iostream>
using namespace std;

int islandPerimeter(const vector<vector<int>>& g) {
    int R = g.size(), C = R ? g[0].size() : 0, per = 0;
    for (int r = 0; r < R; ++r)
        for (int c = 0; c < C; ++c)
            if (g[r][c] == 1) {
                per += 4;
                if (c + 1 < C && g[r][c + 1] == 1) per -= 2;
                if (r + 1 < R && g[r + 1][c] == 1) per -= 2;
            }
    return per;
}

int main() {
    vector<vector<int>> grid = {{0,1,1,0},{1,1,1,0},{0,1,1,0},{0,0,1,0}};
    cout << islandPerimeter(grid) << endl;
    return 0;
}
