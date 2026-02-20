// Day 1106: Count buildings (east->west) with a clear sunset view to the west.
// A building sees west if taller than all to its west; scan from west end, track max.
// Time: O(N) single pass. Space: O(1).
#include <bits/stdc++.h>
using namespace std;

int sunsetViews(const vector<int>& h){
    int count = 0, maxSoFar = INT_MIN;
    for (int i = (int)h.size()-1; i >= 0; i--)   // west -> east
        if (h[i] > maxSoFar) { count++; maxSoFar = h[i]; }
    return count;
}

int main(){
    cout << sunsetViews({3,7,8,3,6,1}) << "\n"; // 3
    return 0;
}
