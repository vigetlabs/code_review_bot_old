const path = require('path')
const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const CopyPlugin = require('copy-webpack-plugin')

module.exports = {
    entry: './client/js/index.js',
    plugins: [
        new CleanWebpackPlugin(),
        new CopyPlugin([
            { from: 'icons/*', context: './client/' },
            { from: 'images/*', context: './client/' },
        ]),
    ],
    output: {
        filename: 'main.js',
        path: path.resolve(__dirname, 'public'),
    },
    module: {
        rules: [
            {
                test: /\.(png|svg|jpg|gif)$/,
                use: ['file_loader']
            }
        ]
    }
}
