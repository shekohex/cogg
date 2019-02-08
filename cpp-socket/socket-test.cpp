#include "gen/userping.pb.h"
#include <arpa/inet.h> //inet_addr
#include <iomanip>
#include <iostream>     //cout
#include <netdb.h>      //hostent
#include <stdio.h>      //printf
#include <string.h>     //strlen
#include <string>       //string
#include <sys/socket.h> //socket
using namespace std;

/**
 * TCP Client class
 */
class tcp_client {
private:
  int sock;
  std::string address;
  int port;
  struct sockaddr_in server;

public:
  tcp_client();
  bool conn(string, int);
  bool send_data(char *data, uint size);
  string receive(int);
};

tcp_client::tcp_client() {
  sock = -1;
  port = 0;
  address = "";
}

/**
 * Connect to a host on a certain port number
 */
bool tcp_client::conn(string address, int port) {
  // create socket if it is not already created
  if (sock == -1) {
    // Create socket
    sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock == -1) {
      perror("Could not create socket");
    }

    cout << "Socket created\n";
  } else { /* OK , nothing */
  }

  // setup address structure
  if (inet_addr(address.c_str()) == -1) {
    struct hostent *he;
    struct in_addr **addr_list;

    // resolve the hostname, its not an ip address
    if ((he = gethostbyname(address.c_str())) == NULL) {
      // gethostbyname failed
      herror("gethostbyname");
      cout << "Failed to resolve hostname\n";

      return false;
    }

    // Cast the h_addr_list to in_addr , since h_addr_list also has the ip
    // address in long format only
    addr_list = (struct in_addr **)he->h_addr_list;

    for (int i = 0; addr_list[i] != NULL; i++) {
      // strcpy(ip , inet_ntoa(*addr_list[i]) );
      server.sin_addr = *addr_list[i];

      cout << address << " resolved to " << inet_ntoa(*addr_list[i]) << endl;

      break;
    }
  }

  // plain ip address
  else {
    server.sin_addr.s_addr = inet_addr(address.c_str());
  }

  server.sin_family = AF_INET;
  server.sin_port = htons(port);

  // Connect to remote server
  if (connect(sock, (struct sockaddr *)&server, sizeof(server)) < 0) {
    perror("connect failed.... Error");
    return 1;
  }

  cout << "Connected\n";
  return true;
}

/**
 * Send data to the connected host
 */
bool tcp_client::send_data(char *data, uint size) {
  // Send some data
  if (send(sock, data, size, 0) < 0) {
    perror("Send failed : ");
    return false;
  }
  cout << "Data send\n";

  return true;
}

/**
 * Receive data from the connected host
 */
string tcp_client::receive(int size = 512) {
  char buffer[size];
  string reply;

  // Receive a reply from the server
  if (recv(sock, buffer, sizeof(buffer), 0) < 0) {
    puts("recv failed");
  }

  reply = buffer;
  return reply;
}
void hexDump(const char *desc, const char *addr, int len) {
  int i;
  unsigned char buff[17];
  unsigned char *pc = (unsigned char *)addr;

  // Output description if given.
  if (desc != NULL)
    printf("+--- Header Packet [%s] Length [%d] ----+\n", desc, len);

  if (len == 0) {
    printf("  ZERO LENGTH\n");
    return;
  }
  if (len < 0) {
    printf("  NEGATIVE LENGTH: %i\n", len);
    return;
  }

  // Process every byte in the data.
  for (i = 0; i < len; i++) {
    // Multiple of 16 means new line (with line offset).

    if ((i % 16) == 0) {
      // Just don't print ASCII for the zeroth line.
      if (i != 0)
        printf("  %s\n", buff);

      // Output the offset.
      printf("| %04x ", i);
    }

    // Now the hex code for the specific character.
    printf(" %02x", pc[i]);

    // And store a printable ASCII character for later.
    if ((pc[i] < 0x20) || (pc[i] > 0x7e))
      buff[i % 16] = '.';
    else
      buff[i % 16] = pc[i];
    buff[(i % 16) + 1] = '\0';
  }

  // Pad out last line if not exactly 16 characters.
  while ((i % 16) != 0) {
    printf("   ");
    i++;
  }

  // And print the final ASCII bit.
  printf("  %s\n", buff);
  printf("+--- Footer Packet [%s] Length [%d] ----+\n", desc, len);
}

int main(int argc, char *argv[]) {
  GOOGLE_PROTOBUF_VERIFY_VERSION;
  tcp_client c;
  // connect to host
  c.conn("127.0.0.1", 8090);
  auto packet = cogg::UserPing::default_instance();
  packet.set_packetid(1);
  packet.set_username("Shady");
  size_t packet_size = packet.ByteSize();
  auto msg_size = packet_size + sizeof(uint8_t) + sizeof(char);
  char msg[msg_size];
  msg[0] = (uint8_t)1; // packet id
  void *buffer = malloc(packet_size);
  packet.SerializeToArray(buffer, packet_size);
  memcpy(msg + sizeof(uint8_t), buffer, packet_size);
  msg[msg_size - sizeof(char)] = '\0';
  // send some data
  hexDump("UserPing", msg, msg_size);
  c.send_data(msg, msg_size);
  cout << "Message Sent!" << endl;
  google::protobuf::ShutdownProtobufLibrary();
  // done
  return 0;
}
