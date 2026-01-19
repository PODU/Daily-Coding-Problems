// Brick wall: count edge offsets (cumulative sums excluding last) via hashmap.
// Fewest cuts = numRows - maxAlignedEdges. Time O(total bricks), Space O(distinct edges).
#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;

int leastBricks(const vector<vector<int>>& wall) {
    unordered_map<long long, int> freq;
    int best = 0;
    for (const auto& row : wall) {
        long long sum = 0;
        for (size_t i = 0; i + 1 < row.size(); ++i) {
            sum += row[i];
            best = max(best, ++freq[sum]);
        }
    }
    return (int)wall.size() - best;
}

int main() {
    vector<vector<int>> wall = {{3,5,1,1},{2,3,3,2},{5,5},{4,4,2},{1,3,3,3},{1,1,6,1,1}};
    cout << leastBricks(wall) << endl;
    return 0;
}
