// Iterate counting integers; pick those whose digit sum == 10. O(answer) time, O(1) space.
#include <iostream>
using namespace std;
int digitSum(long long x){int s=0;while(x){s+=x%10;x/=10;}return s;}
long long nthPerfect(int n){long long x=0;int c=0;while(c<n){++x;if(digitSum(x)==10)++c;}return x;}
int main(){cout<<nthPerfect(1)<<"\n"<<nthPerfect(2)<<"\n";return 0;}
