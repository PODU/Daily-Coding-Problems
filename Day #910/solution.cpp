// Minimum jumps to reach end: greedy BFS-by-levels tracking current reach and farthest reach.
// Bump jumps when current window ends. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int minJumps(const vector<int>& nums) {
    int n = nums.size();
    if (n <= 1) return 0;
    int jumps = 0, curEnd = 0, farthest = 0;
    for (int i = 0; i < n - 1; i++) {
        farthest = max(farthest, i + nums[i]);
        if (i == curEnd) { jumps++; curEnd = farthest; }
    }
    return jumps;
}

int main() {
    vector<int> nums = {6, 2, 4, 0, 5, 1, 1, 4, 2, 9};
    cout << minJumps(nums) << endl;
    return 0;
}
