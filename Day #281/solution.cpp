// Day 281: Fewest bricks cut by a vertical line. Count prefix-sum edge positions;
// answer = rows - max edge frequency. Time O(total bricks), Space O(distinct edges).
#include <bits/stdc++.h>
using namespace std;

int fewestCuts(const vector<vector<int>>& wall) {
    unordered_map<long long, int> edge;
    int best = 0;
    for (const auto& row : wall) {
        long long sum = 0;
        for (size_t i = 0; i + 1 < row.size(); i++) { // skip the far edge
            sum += row[i];
            best = max(best, ++edge[sum]);
        }
    }
    return (int)wall.size() - best;
}

int main() {
    vector<vector<int>> wall = {
        {3, 5, 1, 1}, {2, 3, 3, 2}, {5, 5},
        {4, 4, 2}, {1, 3, 3, 3}, {1, 1, 6, 1, 1}
    };
    cout << fewestCuts(wall) << "\n"; // 2
    return 0;
}
