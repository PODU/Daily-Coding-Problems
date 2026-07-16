// Square root via Newton's method. ~O(log(1/eps)) iterations, O(1) space.
public class Solution {
    static double mysqrt(double n){
        if(n < 0) throw new IllegalArgumentException("negative");
        if(n == 0) return 0;
        double x = n;
        for(int i = 0; i < 200; i++){
            double nx = 0.5 * (x + n / x);
            if(Math.abs(nx - x) < 1e-15){ x = nx; break; }
            x = nx;
        }
        return x;
    }
    public static void main(String[] args){
        double n = 9;
        double r = mysqrt(n);
        if(Math.abs(r - Math.rint(r)) < 1e-9) System.out.println((long)Math.rint(r));
        else System.out.println(r);
    }
}
