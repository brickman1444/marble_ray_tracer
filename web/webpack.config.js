const path = require('path');

const mainConfig = {
    entry: './lib/main.js',
    output: {
        path: path.resolve(__dirname, 'public'),
        filename: 'bundle.js',
    },
    mode: 'production'
};

const workerConfig = {
    entry: './lib/worker.js',
    output: {
        path: path.resolve(__dirname, 'public'),
        filename: 'worker.js',
    },
    mode: 'production',
    target: "webworker"
};

module.exports = [mainConfig, workerConfig];