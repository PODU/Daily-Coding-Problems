// Max weight path top->bottom in triangle, bottom-up DP collapsing rows. O(n) space.
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

int maxPath(vector<vector<int>> t) {
    vector<int> dp = t.back();
    for (int i = (int)t.size() - 2; i >= 0; --i)
        for (int j = 0; j <= i; ++j)
            dp[j] = t[i][j] + max(dp[j], dp[j + 1]);
    return dp[0];
}

int main() {
    cout << maxPath({{1}, {2, 3}, {1, 5, 1}}) << "\n";
    return 0;
}
