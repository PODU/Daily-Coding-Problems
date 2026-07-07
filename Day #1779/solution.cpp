// Minimum coins for {1,5,10,25} via greedy (optimal for this canonical system).
// Time: O(1), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int minCoins(int n) {
    return n/25 + (n%25)/10 + (n%25%10)/5 + (n%25%10%5);
}

int main() {
    cout << minCoins(16) << "\n";
    return 0;
}
