# import http.server
# import socketserver

# PORT = 8000

# Handler = http.server.SimpleHTTPRequestHandler
# Handler.extensions_map.update({
#       ".js": "text/javascript",
# });

# httpd = socketserver.TCPServer(("127.0.0.1", PORT), Handler)
# httpd.serve_forever()

from flask import Flask, send_file, render_template, Response
app = Flask(__name__, template_folder='.')

@app.route('/')
def hello_world():
	return render_template('index.html')

@app.route('/<path:file>')
def pkg(file):
	if file.endswith('.js'):
		return send_file(f"./{file}", mimetype='text/javascript')
	elif file.endswith('.wasm'):
		return send_file(f"./{file}", mimetype='application/wasm')
	elif file.endswith('.png'):
		return send_file(f"./{file}", mimetype='image/png')
	elif file.endswith('.css'):
		return send_file(f"./{file}", mimetype='text/css')
	else:
		return Response("Not Found", status=404)


# @app.route('/brotli-compression/<file>')
# def brotli(file):
# 	if file.endswith('.js'):
# 		return send_file(f"./brotli-compression/{file}", mimetype='text/javascript')
# 	elif file.endswith('.wasm'):
# 		return send_file(f"./brotli-compression/{file}", mimetype='application/wasm')
# 	else:
# 		return send_file(f'./brotli-compression/{file}')

if __name__ == '__main__':
	app.run(debug=True)
