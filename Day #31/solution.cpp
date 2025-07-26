// Edit Distance via DP. Time O(m*n), Space O(min(m,n)) using two rolling rows.
#include <bits/stdc++.h>
using namespace std;

int editDistance(const string& a, const string& b) {
    if (a.size() < b.size()) return editDistance(b, a);
    int m = a.size(), n = b.size();
    vector<int> prev(n + 1), cur(n + 1);
    for (int j = 0; j <= n; j++) prev[j] = j;
    for (int i = 1; i <= m; i++) {
        cur[0] = i;
        for (int j = 1; j <= n; j++) {
            if (a[i - 1] == b[j - 1]) cur[j] = prev[j - 1];
            else cur[j] = 1 + min({prev[j - 1], prev[j], cur[j - 1]});
        }
        swap(prev, cur);
    }
    return prev[n];
}

int main() {
    cout << editDistance("kitten", "sitting") << "\n";
    return 0;
}
