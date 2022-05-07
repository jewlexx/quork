#include <unistd.h>

int IsElevated() {
  uid_t uid = geteuid();

  if (uid == 0) {
    return 1;
  } else {
    return 0;
  }
}