// Day 486: Celebrity problem.
// Two-pointer elimination: one candidate survives in O(n) knows() calls, then
// verify in O(n). Total O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

// mock relation matrix: knows[a][b] == 1 means a knows b
static vector<vector<int>> KNOWS;
bool knows(int a, int b) { return KNOWS[a][b] == 1; }

int findCelebrity(int n) {
    int candidate = 0;
    for (int i = 1; i < n; ++i)
        if (knows(candidate, i)) candidate = i;
    // verify candidate knows no one and everyone knows candidate
    for (int i = 0; i < n; ++i) {
        if (i == candidate) continue;
        if (knows(candidate, i) || !knows(i, candidate)) return -1;
    }
    return candidate;
}

int main() {
    // person 2 is the celebrity: knows nobody, everyone knows them
    KNOWS = {
        {0, 1, 1, 0},
        {1, 0, 1, 1},
        {0, 0, 0, 0},
        {1, 1, 1, 0},
    };
    cout << findCelebrity(4) << "\n"; // 2
    return 0;
}
