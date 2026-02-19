// Day 1102: Min total movement to seat people contiguously (order preserved).
// person i must land at start+i; minimize sum|pos[i]-(start+i)| => shift b[i]=pos[i]-i
// to its median. Time: O(N). Space: O(M).
#include <bits/stdc++.h>
using namespace std;

long long minCost(const vector<int>& seats){
    vector<long long> b;
    for (int i = 0, p = 0; i < (int)seats.size(); i++)
        if (seats[i] == 1) b.push_back(i - (p++)); // pos - index_among_people
    if (b.empty()) return 0;
    nth_element(b.begin(), b.begin()+b.size()/2, b.end());
    long long med = b[b.size()/2], cost = 0;
    for (long long x : b) cost += llabs(x - med);
    return cost;
}

int main(){
    cout << minCost({0,1,1,0,1,0,0,0,1}) << "\n"; // 5
    return 0;
}
