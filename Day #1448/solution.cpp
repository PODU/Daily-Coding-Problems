// Day 1448: Fewest bricks cut by a vertical line through a brick wall.
// Count internal edge positions via prefix sums; answer = rows - maxEdgeCount.
// Time O(total bricks), Space O(distinct edges).
#include <bits/stdc++.h>
using namespace std;

int leastBricks(const vector<vector<int>>& wall) {
    unordered_map<long long,int> edges; // edge position -> count
    int best = 0;
    for (const auto& row : wall) {
        long long pos = 0;
        for (size_t i = 0; i + 1 < row.size(); i++) { // skip far right edge
            pos += row[i];
            best = max(best, ++edges[pos]);
        }
    }
    return (int)wall.size() - best;
}

int main() {
    vector<vector<int>> wall = {
        {3,5,1,1},{2,3,3,2},{5,5},{4,4,2},{1,3,3,3},{1,1,6,1,1}
    };
    cout << leastBricks(wall) << "\n"; // 2
    return 0;
}
