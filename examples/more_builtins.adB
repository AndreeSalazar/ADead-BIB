# more_builtins.adB - Más funciones built-in
# clamp, sign, even, odd, sqr, cube, inc, dec

def main():
    print("=== More Built-ins ===")
    
    # clamp
    c = clamp(150, 0, 100)
    print("clamp(150,0,100):")
    print(c)
    
    # sqr
    s = sqr(7)
    print("sqr(7)=49:")
    print(s)
    
    # cube
    cu = cube(3)
    print("cube(3)=27:")
    print(cu)
    
    # even/odd
    e = even(10)
    print("even(10)=1:")
    print(e)
    
    o = odd(7)
    print("odd(7)=1:")
    print(o)
    
    # inc/dec
    i = inc(5)
    print("inc(5)=6:")
    print(i)
    
    d = dec(10)
    print("dec(10)=9:")
    print(d)
    
    print("=== Done ===")
