from ctypes import *
import ctypes as ct
import os

dll_name = "target/debug/libppm.so"
dllabspath = os.path.dirname(os.path.abspath(__file__)) + os.path.sep + dll_name
print(dllabspath)
myDll = CDLL(dllabspath)

# create Pixel
# Returns Void (Pixel)
myDll.create_pixel.argtypes = [ct.c_uint8, ct.c_uint8, ct.c_uint8]
myDll.create_pixel.restype = ct.c_void_p

# invert Pixel
myDll.invert_pixel.argtypes = [ct.c_void_p]

# Python wrapper to create a Pixel with function
def create_pixel(r, g, b):
    rPy = ct.c_uint8(r)
    gPy = ct.c_uint8(g)
    bPy = ct.c_uint8(b)
    return myDll.create_pixel(rPy, gPy, bPy)

# Python wrapper to invert a Pixel with function
def invert_pixel(pixel_pointer):
    return myDll.invert_pixel(pixel_pointer)

p = create_pixel(10, 10, 10)
print(p)
invert_pixel(p)