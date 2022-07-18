namespace network
{
    enum class INTERNET_STATUS
    {
        CONNECTED = 0,
        DISCONNECTED = 1,
        CONNECTED_TO_LOCAL = 2,
        CONNECTION_ERROR = 3
    };

    INTERNET_STATUS IsConnectedToNetwork();

}