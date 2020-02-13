from cffi import FFI

ffi = FFI()
ffi.cdef('int double(int);')

C = ffi.dlopen("./target/debug/libppm.so")

print(C.double(9))
