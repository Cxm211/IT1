#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void test(const PublicKey *pk, const SecretKey *sk);

}  // extern "C"
