#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

#if defined(FOO)
constexpr static const int32_t DOES_WORK = 1;
#endif

#if defined(BAR)
constexpr static const int32_t DOES_WORK = 2;
#endif

#if defined(FOO)
constexpr static const int32_t DOES_NOT_WORK = 1;
#endif

#if defined(BAR)
constexpr static const int32_t DOES_NOT_WORK = 2;
#endif

#if defined(FOO)
struct DoesWork {

};
#endif

#if defined(BAR)
struct DoesWork {

};
#endif

#if defined(FOO)
struct DoesNotWork {

};
#endif

#if defined(BAR)
struct DoesNotWork {

};
#endif

extern "C" {

#if defined(FOO)
void does_work(const DoesWork *dw);
#endif

#if defined(BAR)
void does_work(const DoesWork *dw);
#endif

#if defined(FOO)
void does_not_work(const DoesNotWork *dnw);
#endif

#if defined(BAR)
void does_not_work(const DoesNotWork *dnw);
#endif

}  // extern "C"
