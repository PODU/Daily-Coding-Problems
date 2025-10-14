// Day 429: kth row of Pascal's triangle (0-indexed: row 0 = [1]).
// Multiplicative recurrence row[j] = row[j-1]*(k-j+1)/j. Time O(k), Space O(k).
#include <bits/stdc++.h>
using namespace std;

int main() {
    int k = 4;
    vector<long long> row(k + 1, 1);
    for (int j = 1; j <= k; j++) row[j] = row[j - 1] * (k - j + 1) / j;
    for (int j = 0; j <= k; j++) {
        cout << row[j];
        if (j < k) cout << " ";
    }
    cout << endl;
    return 0;
}
