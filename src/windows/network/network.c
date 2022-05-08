#ifdef _WIN32
#include <netlistmgr.h>

int IsDisconnected() { return NLM_CONNECTIVITY_DISCONNECTED; }

int HasTraffic() { return 0; }

#endif