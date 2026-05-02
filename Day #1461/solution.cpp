// Egg drop: DP on trials. f(t,e)=f(t-1,e-1)+f(t-1,e)+1 = max floors with t trials, e eggs.
// Smallest t with f(t,N)>=k. Time O(N*answer), Space O(N).
#include <bits/stdc++.h>
using namespace std;

// Minimum worst-case trials for N eggs and k floors.
int eggDrop(int N, int k) {
    if (k == 0) return 0;
    vector<long long> f(N + 1, 0); // floors solvable with current t trials, e eggs
    int t = 0;
    while (f[N] < k) {
        ++t;
        // update in decreasing e so f[e-1] is previous trial's value
        for (int e = N; e >= 1; --e) {
            f[e] = f[e] + f[e - 1] + 1;
        }
    }
    return t;
}

int main() {
    cout << eggDrop(1, 5) << '\n';
    return 0;
}
