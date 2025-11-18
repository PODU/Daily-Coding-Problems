// Brick wall: hashmap of prefix-sum edge positions (excluding full-width edge).
// Answer = numRows - maxEdgeCount. Time O(total bricks), Space O(distinct edges).
#include <bits/stdc++.h>
using namespace std;

int leastBricks(vector<vector<int>>& wall) {
    unordered_map<long long, int> edges;
    int maxCount = 0;
    for (auto& row : wall) {
        long long sum = 0;
        for (int i = 0; i + 1 < (int)row.size(); i++) {
            sum += row[i];
            maxCount = max(maxCount, ++edges[sum]);
        }
    }
    return (int)wall.size() - maxCount;
}

int main() {
    vector<vector<int>> wall = {{3,5,1,1},{2,3,3,2},{5,5},{4,4,2},{1,3,3,3},{1,1,6,1,1}};
    cout << leastBricks(wall) << "\n";
    return 0;
}
