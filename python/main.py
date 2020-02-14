from ppm import *

p1 = Pixel(10, 30, 255)
p2 = Pixel(20, 40, 0)
print("p1 = " + str(p1))
print("p2 = " + str(p2))
p3 = myLib.pixel_avg(p1, p2)
print("Average of p1 and p2 : p3 = " + str(p3))


p = Pixel(0, 0, 0)
print("\nInverting " + str(p))
print(pixel_inv(p))

p = Pixel(10, 20, 30)
print("\nTo Gray " + str(p))
print(pixel_to_gray(p))


img = open_image("/home/antoine/ESGI/test/RustProject/ppm/img.ppm")
print(type(img))

#display_image_in_terminal(img)