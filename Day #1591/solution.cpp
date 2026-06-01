// Subset Sum: boolean DP over reachable sums with parent tracking; reconstruct one subset.
// Time O(n*k), Space O(n*k).
#include <bits/stdc++.h>
using namespace std;

vector<int> subsetSum(const vector<int>& S, int k) {
    int n = S.size();
    // reach[i][s] = sum s achievable using first i items
    vector<vector<char>> reach(n + 1, vector<char>(k + 1, 0));
    reach[0][0] = 1;
    for (int i = 1; i <= n; ++i) {
        for (int s = 0; s <= k; ++s) {
            if (reach[i - 1][s]) reach[i][s] = 1;
            if (s >= S[i - 1] && reach[i - 1][s - S[i - 1]]) reach[i][s] = 1;
        }
    }
    if (!reach[n][k]) return {}; // sentinel empty == impossible (target>0)
    vector<int> chosen;
    int s = k;
    for (int i = n; i >= 1; --i) {
        // prefer taking item i if it leads to a valid path
        if (s >= S[i - 1] && reach[i - 1][s - S[i - 1]]) {
            chosen.push_back(S[i - 1]);
            s -= S[i - 1];
        }
    }
    return chosen; // may be empty only if impossible (target>0)
}

int main() {
    vector<int> S = {12, 1, 61, 5, 9, 2};
    int k = 24;
    vector<int> sub = subsetSum(S, k);
    cout << "[";
    for (size_t i = 0; i < sub.size(); ++i) {
        cout << sub[i];
        if (i + 1 < sub.size()) cout << ", ";
    }
    cout << "]\n";
    int total = 0;
    for (int x : sub) total += x;
    cout << "Sum = " << total << "\n";
    return 0;
}
