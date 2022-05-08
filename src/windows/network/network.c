#include <netlistmgr.h>

int IsDisconnected() {
  INetwork net;
  VARIANT_BOOL connected;
  net.lpVtbl->get_IsConnected(net.lpVtbl, &connected);
}

int HasTraffic() { return 0; }
