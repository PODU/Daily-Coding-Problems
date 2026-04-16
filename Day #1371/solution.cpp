// Assign mice to holes minimizing max distance: sort both, pair in order.
// Time O(n log n), Space O(1) extra.
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
    cout << minLastMouse({1, 4, 9, 15}, {10, -5, 0, 16}) << "\n";
    return 0;
}
