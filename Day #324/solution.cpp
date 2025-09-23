// Assign mice to holes minimizing max distance: sort both, pair i-th, answer = max|mice[i]-holes[i]|.
// Time: O(N log N), Space: O(1) extra.
#include <bits/stdc++.h>
using namespace std;

int minMaxDistance(vector<int> mice, vector<int> holes) {
    sort(mice.begin(), mice.end());
    sort(holes.begin(), holes.end());
    int ans = 0;
    for (size_t i = 0; i < mice.size(); i++)
        ans = max(ans, abs(mice[i] - holes[i]));
    return ans;
}

int main() {
    vector<int> mice = {1, 4, 9, 15};
    vector<int> holes = {10, -5, 0, 16};
    cout << minMaxDistance(mice, holes) << "\n";
    return 0;
}
