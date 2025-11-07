// Jump Game: greedy tracking farthest reachable index. Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

bool canJump(const vector<int>& nums) {
    int farthest = 0;
    for (int i = 0; i < (int)nums.size(); ++i) {
        if (i > farthest) return false;
        farthest = max(farthest, i + nums[i]);
    }
    return true;
}

int main() {
    cout << (canJump({2, 0, 1, 0}) ? "True" : "False") << "\n";
    cout << (canJump({1, 1, 0, 1}) ? "True" : "False") << "\n";
    return 0;
}
