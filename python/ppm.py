from ctypes import *
import ctypes as ct
import os

dll_name = "ppm/target/debug/libppm.so"
path = os.path.dirname(os.path.abspath(__file__))
path = os.path.split(path)[0]
dllabspath = path + os.path.sep + dll_name
myLib = CDLL(dllabspath)


#Pixel wrapper for rust struct Pixel
class Pixel(ct.Structure):
    _fields_ = [("r", ct.c_uint8), ("g", ct.c_uint8), ("b", ct.c_uint8)]

    def __str__(self):
        return "Pixel ({},{},{})".format(self.r, self.g, self.b)


# Makes an average of pixel
myLib.pixel_avg.argtypes = (Pixel, Pixel)
myLib.pixel_avg.restype = Pixel

#Invert Pixel
myLib.pixel_inv.argtypes = [ct.c_void_p]
myLib.pixel_inv.restype = Pixel

#To Gray Pixel
myLib.pixel_to_gray.argtypes = [ct.c_void_p]
myLib.pixel_to_gray.restype = Pixel

def pixel_inv(pixel):
    p = ct.pointer(pixel)
    return myLib.pixel_inv(p)

def pixel_to_gray(pixel):
    p = ct.pointer(pixel)
    return myLib.pixel_to_gray(p)


