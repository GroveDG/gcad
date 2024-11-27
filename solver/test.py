from geo import *

a = Vec(1,1)
b = Vec(2,2)
c = Vec(3,0)

# circle = Circle(a, 10)
# ray = Ray(c, Vec(0,-1))
# print(meet(circle, ray))

l1 = Ray.join(Vec(0,0), Vec(0,1))
print(l1)
l2 = Ray.join(Vec(1,0), Vec(1,1))
# print(meet(l1,l2))