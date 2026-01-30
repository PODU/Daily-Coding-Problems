// Day 989: Deduce coin denominations from a ways-to-make-change array.
// Walk amounts; whenever target[i] exceeds reconstructed ways, i is a coin and we fold it into the DP.
// O(N^2) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

vector<int> findDenominations(const vector<long long>& target) {
    int n = target.size();
    vector<long long> have(n, 0);
    have[0] = 1;                     // one way to make 0 with no coins
    vector<int> coins;
    for (int i = 1; i < n; i++) {
        if (target[i] > have[i]) {   // unaccounted combinations => i is a denomination
            coins.push_back(i);
            for (int j = i; j < n; j++) have[j] += have[j - i];
        }
    }
    return coins;
}

int main() {
    vector<long long> target = {1, 0, 1, 1, 2};
    vector<int> coins = findDenominations(target);
    for (size_t k = 0; k < coins.size(); k++) {
        cout << coins[k];
        if (k + 1 < coins.size()) cout << ", ";
    }
    cout << endl; // expected: 2, 3, 4
    return 0;
}
