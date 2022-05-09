#include <windows.h>

int Clipboard() {
  BOOL success = OpenClipboard(NULL);

  if (!success) {
    return -1;
  }
}