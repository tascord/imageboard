const express = require('express');
const http = require('http');

const app = express();
const port = 3000;

app.set('port', port);
var server = http.createServer(app);
server.listen(port, '127.0.1.3');

var index_handler = function(req, res) {
    res.send(req.query);
}

app.get('/', index_handler);