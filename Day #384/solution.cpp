// Min coins via bottom-up DP. Returns -1 (null) if unreachable.
// Time: O(amount * |coins|), Space: O(amount).
#include <bits/stdc++.h>
using namespace std;

int minCoins(const vector<int>& coins, int amount){
    const int INF = INT_MAX;
    vector<int> dp(amount+1, INF);
    dp[0] = 0;
    for(int a=1;a<=amount;a++)
        for(int c : coins)
            if(c <= a && dp[a-c] != INF) dp[a] = min(dp[a], dp[a-c]+1);
    return dp[amount] == INF ? -1 : dp[amount];
}

void show(const vector<int>& coins, int amount){
    int r = minCoins(coins, amount);
    if(r < 0) cout << "null\n"; else cout << r << "\n";
}

int main(){
    show({1,5,10}, 56);
    show({5,8}, 15);
    return 0;
}
