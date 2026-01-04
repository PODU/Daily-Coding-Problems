// Day 855: count N-Queens solutions via backtracking with bitmasks for column/diagonals.
// O(N!) worst case, O(N) space. Bitmask makes conflict checks O(1).
#include <bits/stdc++.h>
using namespace std;

int n, full;
long long count_ = 0;

void solve(int cols, int diag1, int diag2){
    if(cols == full){ count_++; return; }
    int avail = full & ~(cols | diag1 | diag2);
    while(avail){
        int p = avail & (-avail);        // lowest available column
        avail -= p;
        solve(cols | p, (diag1 | p) << 1, (diag2 | p) >> 1);
    }
}

long long countNQueens(int N){
    n = N; full = (1 << N) - 1; count_ = 0;
    solve(0, 0, 0);
    return count_;
}

int main(){
    for(int N = 1; N <= 8; ++N)
        cout << "N=" << N << ": " << countNQueens(N) << "\n";
    // N=8: 92
    return 0;
}
