// Sort mice and holes, pair by index, answer = max |mice[i]-holes[i]|. Time O(n log n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> mice = {1, 4, 9, 15};
    vector<int> holes = {10, -5, 0, 16};
    sort(mice.begin(), mice.end());
    sort(holes.begin(), holes.end());
    int ans = 0;
    for (size_t i = 0; i < mice.size(); ++i)
        ans = max(ans, abs(mice[i] - holes[i]));
    cout << ans << endl;
    return 0;
}
