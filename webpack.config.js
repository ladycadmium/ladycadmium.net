const path = require('path');

module.exports = {
  mode: 'production', 
  entry: './src/index.ts', 

  module: {
    rules: [
      {
        // For all .ts files, use the ts-loader
        test: /\.ts$/, 
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  
  resolve: {
    extensions: ['.ts', '.js'], 
  },
  
  output: {
    filename: 'index.js',
    path: path.resolve(__dirname, 'dist'), 
    clean: true, 
  },
};