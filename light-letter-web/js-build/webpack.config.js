const path = require('path');

module.exports = {
  mode: 'production',
  entry: './js-build/index.js',
  output: {
    path: path.resolve(__dirname, '../dist'),
    filename: 'light_letter_web.js',
    library: '__light_letter__',
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /(node_modules|bower_components)/,
        loader: 'babel-loader',
        options: {
          presets: ['@babel/preset-env'],
        },
      },
      {
        test: /.wasm$/,
        loader: 'webassembly-loader',
        options: { name: '[name][ext]' },
      }
    ]
  },
  optimization: {
    minimize: false,
  },
};
