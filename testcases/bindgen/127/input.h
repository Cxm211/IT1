struct cmdline_token_hdr {
	struct cmdline_token_ops *ops;
};

struct cmdline_token_ops {
  int foo;
};