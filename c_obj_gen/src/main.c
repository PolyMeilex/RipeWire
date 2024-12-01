#include <getopt.h>
#include <stdint.h>

#include "./rust_out.h"
#include "./json_out.h"

int main(void) {
  /* rust_out_run(); */
  json_out_run();

  return 0;
}
