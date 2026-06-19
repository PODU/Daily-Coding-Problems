# Day 1692: Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
# Bonus: hands overlap (angle 0) 11 times per 12 hours, at t = (12/11)*k hours for k=0..10.

def clock_angle(hh, mm):
    minute_angle = mm * 6.0
    hour_angle = (hh % 12) * 30.0 + mm * 0.5
    diff = abs(hour_angle - minute_angle)
    angle = min(diff, 360.0 - diff)
    return int(round(angle))


if __name__ == "__main__":
    t = "3:15"
    hh, mm = (int(x) for x in t.split(":"))
    print(clock_angle(hh, mm))
