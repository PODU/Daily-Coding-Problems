// Monte Carlo simulation of two dice stopping games; average rolls. O(trials) time, O(1) space.
// Theory: E[rolls "5 then 6"]=36, E[rolls "5 then 5"]=42. Exact sim values depend on RNG/seed.
#include <bits/stdc++.h>
using namespace std;

int trial(mt19937 &rng, int second){
    uniform_int_distribution<int> d(1,6);
    int rolls=0, prev=0;
    while(true){ int c=d(rng); rolls++; if(prev==5 && c==second) return rolls; prev=c; }
}

int main(){
    mt19937 rng(12345);
    const int T=100000;
    long long s1=0, s2=0;
    for(int i=0;i<T;i++) s1+=trial(rng,6);
    for(int i=0;i<T;i++) s2+=trial(rng,5);
    double e1=(double)s1/T, e2=(double)s2/T;
    printf("Game 1 (five then six) expected rolls: %.2f\n", e1);
    printf("Game 2 (five then five) expected rolls: %.2f\n", e2);
    if(e1<e2) printf("Alice should play Game 1 (five then six), it has lower expected cost.\n");
    else      printf("Alice should play Game 2 (five then five), it has lower expected cost.\n");
    return 0;
}
