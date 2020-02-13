from cffi import FFI

ffi = FFI()
ffi.cdef("""
    int double(int);
""")

C = ffi.dlopen("../ffi/target/debug/ppm.dll")

print(C.double(9))
