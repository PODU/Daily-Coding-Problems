// Expected rounds until one coin remains. Each round a coin survives w.p. 1/2.
// DP recurrence: E(n)*(2^n-1) = 2^n + sum_{k=2..n-1} C(n,k) E(k); E(0)=E(1)=0. O(n^2).
#include <bits/stdc++.h>
using namespace std;

double expectedRounds(int n){
    if(n <= 1) return 0.0;
    vector<vector<double>> C(n+1, vector<double>(n+1, 0));
    for(int i = 0; i <= n; i++){
        C[i][0] = 1;
        for(int j = 1; j <= i; j++) C[i][j] = C[i-1][j-1] + C[i-1][j];
    }
    vector<double> E(n+1, 0.0);
    for(int m = 2; m <= n; m++){
        double pm = pow(2.0, m);
        double sum = pm;
        for(int k = 2; k <= m-1; k++) sum += C[m][k] * E[k];
        E[m] = sum / (pm - 1.0);
    }
    return E[n];
}

int main(){
    int n = 4;
    cout << expectedRounds(n) << "\n"; // ~2.05714 rounds
    return 0;
}
