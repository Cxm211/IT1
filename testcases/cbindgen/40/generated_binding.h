#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

/// The object allocated by an Arc<T>
template<typename T>
struct ArcInner {
  AtomicUsize count;
#if defined(DEFINE_TRACK_ALLOC_SIZE)
  uintptr_t alloc_size
#endif
  ;
  T data;

  bool operator==(const ArcInner& other) const {
    return count == other.count &&
           alloc_size == other.alloc_size &&
           data == other.data;
  }
  bool operator!=(const ArcInner& other) const {
    return count != other.count ||
           alloc_size != other.alloc_size ||
           data != other.data;
  }
};

extern "C" {

void do_smth(ArcInner<uint8_t> t);

}  // extern "C"
