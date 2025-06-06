#ifndef _PACKED_H_
#define _PACKED_H_

#include <stdint.h>

struct mystruct {
	unsigned	a : 2;
	unsigned	b : 4;
	unsigned	c : 3;
	unsigned	d : 5;
	unsigned	e : 2;
	uint16_t	f;
	uint32_t	g;
} __attribute__((packed));

#endif