#define ALIGN_SIZE 64 /* Should be enough for most CPUs */

typedef unsigned short __u16;
typedef unsigned int __u32;

struct tcmu_mailbox {
	__u16 version;
	__u16 flags;
	__u32 cmdr_off;
	__u32 cmdr_size;

	__u32 cmd_head;

	/* Updated by user. On its own cacheline */
	__u32 cmd_tail __attribute__((__aligned__(ALIGN_SIZE)));

} __attribute__((packed));