// Presence bitset: mark each present value, then report unmarked ones.
// Time: O(N), Space: O(N) bits.  (N = 1,000,000)
#include <bits/stdc++.h>
using namespace std;

vector<int> findMissing(const vector<int>& present, int N){
    vector<bool> seen(N+1, false);
    for(int x : present) seen[x] = true;
    vector<int> missing;
    for(int i=1;i<=N;i++) if(!seen[i]) missing.push_back(i);
    return missing;
}

int main(){
    const int N = 1000000;
    // Build the "input": all numbers 1..N except the 1000 multiples of 1000.
    vector<int> present;
    present.reserve(N - 1000);
    for(int i=1;i<=N;i++) if(i % 1000 != 0) present.push_back(i);

    vector<int> missing = findMissing(present, N);
    cout << "Missing count: " << missing.size() << "\n";
    cout << "First 10 missing:";
    for(int i=0;i<10 && i<(int)missing.size();i++) cout << " " << missing[i];
    cout << "\n";
    cout << "Time complexity: O(N), Space complexity: O(N) bits (N = 1,000,000)\n";
    return 0;
}
