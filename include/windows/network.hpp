namespace network
{
    enum class INTERNET_STATUS
    {
        CONNECTED,
        DISCONNECTED,
        CONNECTED_TO_LOCAL,
        CONNECTION_ERROR
    };

    INTERNET_STATUS IsConnectedToNetwork();

}