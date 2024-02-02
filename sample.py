import http.server
import socketserver
import subprocess

PORT = 8000
SCRIPT_PATH = "sh /mnt/root-bg/root/device/lxs-rabbit.sh restart"

class MyHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        try:
            # 执行固定的 shell 脚本
            result = subprocess.run(['bash', SCRIPT_PATH], stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True, check=True)
            output = result.stdout

            # 发送脚本执行结果作为响应
            self.send_response(200)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            self.wfile.write(output.encode('utf-8'))
        except subprocess.CalledProcessError as e:
            self.send_response(500)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            error_message = f'Script execution failed: {e}\n{e.stderr}'
            self.wfile.write(error_message.encode('utf-8'))

Handler = MyHandler

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print("Server started at port", PORT)
    httpd.serve_forever()
