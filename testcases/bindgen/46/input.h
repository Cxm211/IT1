// Extracted from emmintrin.h on macOS (Xcode 10.1)

#define __DEFAULT_FN_ATTRS __attribute__((__always_inline__, __nodebug__, __target__("sse2")))

typedef long long __m128i __attribute__((__vector_size__(16)));
typedef unsigned char __v16qu __attribute__((__vector_size__(16)));

static __inline__ __m128i __DEFAULT_FN_ATTRS
_mm_avg_epu8(__m128i __a, __m128i __b)
{
  typedef unsigned short __v16hu __attribute__ ((__vector_size__ (32)));
  return (__m128i)__builtin_convertvector(
               ((__builtin_convertvector((__v16qu)__a, __v16hu) +
                 __builtin_convertvector((__v16qu)__b, __v16hu)) + 1)
                 >> 1, __v16qu);
}