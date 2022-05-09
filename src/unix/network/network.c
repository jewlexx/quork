#include <NetworkManager.h>
#include <glib.h>
#include <netdb.h>
#include <netinet/in.h>
#include <stdio.h>
#include <sys/socket.h>

int IsConnected() {
  NMClient *client;

  client = nm_client_new(NULL, NULL);

  if (client) {
    printf("NetworkManager version: %s\n", nm_client_get_version(client));
  }

  return 0;
}

int main() {
  NMClient *client;

  client = nm_client_new(NULL, NULL);

  if (client)
    printf("NetworkManager version: %s\n", nm_client_get_version(client));
  return 0;
}
