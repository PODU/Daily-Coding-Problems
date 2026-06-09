// Min subset-sum difference: 0/1 subset-sum DP over reachable sums, pick best<=total/2; backtrack subset. O(n*total) time/space.
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<int> a = {5, 10, 15, 20, 25};
    int n = a.size();
    int total = 0;
    for (int x : a) total += x;

    // dp[i][s] = sum s reachable using first i items
    vector<vector<char>> dp(n + 1, vector<char>(total + 1, 0));
    dp[0][0] = 1;
    for (int i = 1; i <= n; i++)
        for (int s = 0; s <= total; s++) {
            dp[i][s] = dp[i - 1][s];
            if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) dp[i][s] = 1;
        }

    int best = 0;
    for (int s = total / 2; s >= 0; s--)
        if (dp[n][s]) { best = s; break; }

    // Backtrack from last item to first to recover subset A
    vector<int> A;
    vector<char> used(n, 0);
    int s = best;
    for (int i = n; i >= 1; i--) {
        if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) {
            A.push_back(a[i - 1]);
            used[i - 1] = 1;
            s -= a[i - 1];
        }
    }
    sort(A.begin(), A.end());

    vector<int> B;
    for (int i = 0; i < n; i++)
        if (!used[i]) B.push_back(a[i]);

    auto printVec = [](const vector<int>& v) {
        cout << "[";
        for (size_t i = 0; i < v.size(); i++) {
            if (i) cout << ", ";
            cout << v[i];
        }
        cout << "]";
    };

    cout << "Minimum difference: " << (total - 2 * best) << "\n";
    cout << "Subset A: "; printVec(A); cout << "\n";
    cout << "Subset B: "; printVec(B); cout << "\n";
    return 0;
}
