#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

uint32_t allsorts_lookup_glyph_index(unsigned char *subtable, uint32_t char_code);

}  // extern "C"
