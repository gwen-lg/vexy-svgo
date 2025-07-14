import ctypes

# Load the shared library
lib = ctypes.CDLL("../../target/release/libvexy_svgo_ffi.dylib")

# Define the argument and return types for the C functions
lib.vexy_svgo_optimize_default.argtypes = [ctypes.c_char_p]
lib.vexy_svgo_optimize_default.restype = ctypes.POINTER(VexyResult)
lib.vexy_svgo_free_result.argtypes = [ctypes.POINTER(VexyResult)]

class VexyResult(ctypes.Structure):
    _fields_ = [("error_code", ctypes.c_int),
                ("data", ctypes.c_char_p),
                ("data_length", ctypes.c_size_t),
                ("original_size", ctypes.c_size_t),
                ("optimized_size", ctypes.c_size_t),
                ("error_message", ctypes.c_char_p)]

svg = b'<svg><g><rect width="100" height="100"/></g></svg>'

result = lib.vexy_svgo_optimize_default(svg)

if result.contents.error_code == 0:
    print(result.contents.data.decode())
else:
    print(f"Error: {result.contents.error_message.decode()}")

lib.vexy_svgo_free_result(result)
