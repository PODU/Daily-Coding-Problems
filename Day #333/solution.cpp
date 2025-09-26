// Celebrity problem: 2-pass. Pass 1 eliminate to one candidate; pass 2 verify.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

vector<vector<int>> M = {
    {0,1,1,0},  // person0 knows {1,2}
    {0,0,1,0},  // person1 knows {2}
    {0,0,0,0},  // person2 knows {}
    {1,1,1,0}   // person3 knows {0,1,2}
};
bool knows(int a, int b) { return M[a][b] == 1; }

int findCelebrity(int n) {
    int cand = 0;
    for (int i = 1; i < n; i++)
        if (knows(cand, i)) cand = i;
    for (int i = 0; i < n; i++) {
        if (i == cand) continue;
        if (knows(cand, i) || !knows(i, cand)) return -1;
    }
    return cand;
}

int main() {
    cout << findCelebrity(4) << "\n";
    return 0;
}
