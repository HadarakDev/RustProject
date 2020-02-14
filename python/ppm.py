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

    # pixels: Vec<Pixel>,
    # height: usize,
    # width: usize,
#Image wrapper for rust struct Pixel
class Image(ct.Structure):
    _fields_ = [("pixels", ct.c_void_p), ("height", ct.c_size_t), ("width", ct.c_size_t)]

    def __str__(self):
        return "Pixel ({},{},{})".format(self.pixels, self.height, self.width)


# Makes an average of pixel
myLib.pixel_avg.argtypes = (Pixel, Pixel)
myLib.pixel_avg.restype = Pixel

#Invert Pixel
myLib.pixel_inv.argtypes = [ct.c_void_p]
myLib.pixel_inv.restype = Pixel

#To Gray Pixel
myLib.pixel_to_gray.argtypes = [ct.c_void_p]
myLib.pixel_to_gray.restype = Pixel

#Open Image
myLib.open_image.argtypes = [ct.c_char_p]
myLib.open_image.restype = Image

#Display Image
myLib.display_image_in_terminal.argtypes = [Image]

def pixel_inv(pixel):
    p = ct.pointer(pixel)
    return myLib.pixel_inv(p)

def pixel_to_gray(pixel):
    p = ct.pointer(pixel)
    return myLib.pixel_to_gray(p)

def open_image(path):
    b_path = path.encode('utf-8')
    print(b_path)
    return myLib.open_image(b_path)

def display_image_in_terminal(img):
    img.height = int(img.height / 4)
    img.width = int(img.width / 4)
    return myLib.display_image_in_terminal(img)