// header.h

#include <cstdint>
#include <cstdlib>

static const int32_t DOES_NOT_WORK = 1;

static const int32_t DOES_NOT_WORK = 2;

#if defined(FOO)
static const int32_t DOES_WORK = 1;
#endif

#if defined(BAR)
static const int32_t DOES_WORK = 2;
#endif

struct DoesNotWork {

};

struct DoesNotWork {

};

#if defined(FOO)
struct DoesWork {

};
#endif

#if defined(BAR)
struct DoesWork {

};
#endif

extern "C" {

void does_not_work(const DoesNotWork *dnw);

void does_not_work(const DoesNotWork *dnw);

#if defined(FOO)
void does_work(const DoesWork *dw);
#endif

#if defined(BAR)
void does_work(const DoesWork *dw);
#endif

} // extern "C"
