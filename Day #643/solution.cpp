// Day 643: Longest common subsequence of three strings.
// Approach: 3D DP over prefix lengths of a, b, c.
// Time: O(|a|*|b|*|c|), Space: O(|b|*|c|) (two rolling layers).
#include <bits/stdc++.h>
using namespace std;

int lcs3(const string& a, const string& b, const string& c) {
    int A = a.size(), B = b.size(), C = c.size();
    vector<vector<int>> prev(B + 1, vector<int>(C + 1, 0));
    for (int i = 1; i <= A; i++) {
        vector<vector<int>> cur(B + 1, vector<int>(C + 1, 0));
        for (int j = 1; j <= B; j++)
            for (int k = 1; k <= C; k++) {
                if (a[i-1] == b[j-1] && b[j-1] == c[k-1])
                    cur[j][k] = prev[j-1][k-1] + 1;
                else
                    cur[j][k] = max({prev[j][k], cur[j-1][k], cur[j][k-1]});
            }
        prev = move(cur);
    }
    return prev[B][C];
}

int main() {
    cout << lcs3("epidemiologist", "refrigeration",
                 "supercalifragilisticexpialodocious") << "\n"; // 5
    return 0;
}
