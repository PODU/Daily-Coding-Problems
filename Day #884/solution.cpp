// rand5 from rand7 via rejection sampling: keep rand7 values in 1..5. Expected O(1) calls (7/5).
#include <bits/stdc++.h>
using namespace std;

mt19937 rng(12345);

int rand7(){
    uniform_int_distribution<int> d(1, 7);
    return d(rng);
}

int rand5(){
    int x;
    do { x = rand7(); } while(x > 5);
    return x;
}

int main(){
    int counts[6] = {0};
    int trials = 100000;
    for(int i = 0; i < trials; i++) counts[rand5()]++;
    cout << "Distribution over " << trials << " samples:\n";
    for(int v = 1; v <= 5; v++) cout << v << ": " << counts[v] << "\n";
    return 0;
}
