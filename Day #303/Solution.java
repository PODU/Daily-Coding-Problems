// Day 303: Angle between clock hands. O(1).
// Bonus: hands overlap (angle 0) every 12/11 hours (~65.45 min apart).
public class Solution {
    static int clockAngle(int h, int m) {
        double hr = (h % 12) * 30.0 + m * 0.5;
        double mn = m * 6.0;
        double diff = Math.abs(hr - mn);
        diff = Math.min(diff, 360.0 - diff);
        return (int) Math.round(diff);
    }
    public static void main(String[] a) {
        System.out.println(clockAngle(3, 30)); // 75
    }
}
