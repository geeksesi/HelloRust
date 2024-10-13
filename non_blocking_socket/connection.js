const Net = require('net');
const port = 3000;
const host = 'localhost';

// Create a new TCP client.
const client = new Net.Socket();
client.connect({ port: port, host: host }, function () {
    console.log('TCP connection established with the server.');
    client.write('Hello, server.');

});


client.on('data', function (chunk) {
    console.log(`Data received from the server: ${chunk.toString()}.`);

    client.write('Hello, server.');
});

client.on('end', function () {
    console.log('Requested an end to the TCP connection');
});