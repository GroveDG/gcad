from geo import *

a = Vec(1,1)
b = Vec(2,2)
c = Vec(3,0)

l1 = Line.join(a,b)
l2 = Line.join(b,c)

print(meet(l1, l2))