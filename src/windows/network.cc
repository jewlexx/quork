#include "quork/src/windows/network.rs.h"
#include <ObjBase.h> // include the base COM header
#include <Netlistmgr.h>

// Instruct linker to link to the required COM libraries
#pragma comment(lib, "ole32.lib")

namespace network
{
    INTERNET_STATUS IsConnectedToNetwork()
    {
        INTERNET_STATUS connectedStatus = INTERNET_STATUS::CONNECTION_ERROR;
        HRESULT hr = S_FALSE;

        try
        {
            hr = CoInitialize(NULL);
            if (SUCCEEDED(hr))
            {
                INetworkListManager *pNetworkListManager;
                hr = CoCreateInstance(CLSID_NetworkListManager, NULL, CLSCTX_ALL, __uuidof(INetworkListManager), (LPVOID *)&pNetworkListManager);
                if (SUCCEEDED(hr))
                {
                    NLM_CONNECTIVITY nlmConnectivity = NLM_CONNECTIVITY::NLM_CONNECTIVITY_DISCONNECTED;
                    VARIANT_BOOL isConnected = VARIANT_FALSE;
                    hr = pNetworkListManager->get_IsConnectedToInternet(&isConnected);
                    if (SUCCEEDED(hr))
                    {
                        if (isConnected == VARIANT_TRUE)
                            connectedStatus = INTERNET_STATUS::CONNECTED;
                        else
                            connectedStatus = INTERNET_STATUS::DISCONNECTED;
                    }

                    if (isConnected == VARIANT_FALSE && SUCCEEDED(pNetworkListManager->GetConnectivity(&nlmConnectivity)))
                    {
                        if (nlmConnectivity & (NLM_CONNECTIVITY_IPV4_LOCALNETWORK | NLM_CONNECTIVITY_IPV4_SUBNET | NLM_CONNECTIVITY_IPV6_LOCALNETWORK | NLM_CONNECTIVITY_IPV6_SUBNET))
                        {
                            connectedStatus = INTERNET_STATUS::CONNECTED_TO_LOCAL;
                        }
                    }

                    pNetworkListManager->Release();
                }
            }

            CoUninitialize();
        }
        catch (...)
        {
            connectedStatus = INTERNET_STATUS::CONNECTION_ERROR;
        }

        return connectedStatus;
    }
}
