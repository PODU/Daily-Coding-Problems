// Square root via Newton's method. ~O(log(1/eps)) iterations, O(1) space.
#include <bits/stdc++.h>
using namespace std;

double mysqrt(double n){
    if(n < 0) throw runtime_error("negative");
    if(n == 0) return 0;
    double x = n;
    for(int i = 0; i < 200; i++){
        double nx = 0.5 * (x + n / x);
        if(fabs(nx - x) < 1e-15){ x = nx; break; }
        x = nx;
    }
    return x;
}

int main(){
    double n = 9;
    double r = mysqrt(n);
    if(fabs(r - round(r)) < 1e-9) cout << (long long)round(r) << "\n";
    else cout << r << "\n";
    return 0;
}
