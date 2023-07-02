from http.server import BaseHTTPRequestHandler, HTTPServer
import json


class DummyRPCHandler(BaseHTTPRequestHandler):
    def _set_response(self, status_code=200):
        self.send_response(status_code)
        self.send_header('Content-type', 'application/json')
        self.end_headers()

    def do_POST(self):
        content_length = int(self.headers['Content-Length'])
        request_body = self.rfile.read(content_length)
        rpc_request = json.loads(request_body)

        if rpc_request['method'] == 'eth_blockNumber':
            response = {
                "jsonrpc": "2.0",
                "id": rpc_request['id'],
                "result": "0x123456"  # Dummy block number
            }
        else:
            response = {
                "jsonrpc": "2.0",
                "id": rpc_request['id'],
                "error": {
                    "code": -32601,
                    "message": "Method not found"
                }
            }

        self._set_response()
        self.wfile.write(json.dumps(response).encode())

    def do_GET(self):
        self._set_response(404)
        self.wfile.write(b'Not found')


def run_server():
    server_address = ('localhost', 8000)
    httpd = HTTPServer(server_address, DummyRPCHandler)
    print('Dummy RPC server is running on http://localhost:8000...')
    httpd.serve_forever()


if __name__ == '__main__':
    run_server()