// Day 725: Assign mice to holes minimizing the maximum distance any mouse moves.
// Approach: Sort both arrays, pair in order, answer = max |mice[i]-holes[i]|.
// Time: O(n log n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int minLastMouse(vector<int> mice, vector<int> holes) {
    sort(mice.begin(), mice.end());
    sort(holes.begin(), holes.end());
    int ans = 0;
    for (size_t i = 0; i < mice.size(); i++)
        ans = max(ans, abs(mice[i] - holes[i]));
    return ans;
}

int main() {
    vector<int> mice = {1, 4, 9, 15}, holes = {10, -5, 0, 16};
    cout << minLastMouse(mice, holes) << "\n"; // 6
    return 0;
}
