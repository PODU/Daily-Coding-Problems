// Reconstruct coin denominations from a ways-to-make-change array.
// DP coin detection: A[i] > ways[i] means i is a coin. Time O(N^2), Space O(N).
#include <bits/stdc++.h>
using namespace std;

string joinCoins(const vector<int>& c) {
    if (c.empty()) return "";
    if (c.size() == 1) return to_string(c[0]);
    if (c.size() == 2) return to_string(c[0]) + " and " + to_string(c[1]);
    string s;
    for (size_t i = 0; i + 1 < c.size(); ++i) s += to_string(c[i]) + ", ";
    s += "and " + to_string(c.back());
    return s;
}

vector<int> findCoins(const vector<int>& A) {
    int n = A.size();
    vector<long long> ways(n, 0);
    ways[0] = 1;
    vector<int> coins;
    for (int i = 1; i < n; ++i) {
        if (A[i] > ways[i]) {
            coins.push_back(i);
            for (int j = i; j < n; ++j) ways[j] += ways[j - i];
        }
    }
    return coins;
}

int main() {
    vector<int> A = {1, 0, 1, 1, 2};
    cout << joinCoins(findCoins(A)) << "\n";
    return 0;
}
