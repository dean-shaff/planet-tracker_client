// for use with webpack v4
const path = require("path")

const { VueLoaderPlugin } = require('vue-loader')
const MiniCssExtractPlugin = require("mini-css-extract-plugin")

module.exports = {
    mode: 'production',
    entry: [
        "./src/app.js",
        "./src/index.css"
    ],
    output: {
        filename: "bundle.js",
        path: path.resolve(__dirname, "dist")
    },
    module: {
        rules: [
            {
                test: /\.vue$/,
                include: path.resolve(__dirname, "src"),
                use: 'vue-loader'
            },
            {
                test: /\.js$/,
                include: path.resolve(__dirname, "src"),
                loader: "babel-loader"
            },
            {
                test: /\.css$/,
                use:[
                    MiniCssExtractPlugin.loader,
                        // options: {
                        //     publicPath: path.resolve(__dirname, "dist")
                        // }
                    "css-loader"
                ]
            },
            {
                test: /\.(eot|woff|woff2|svg|ttf)([\?]?.*)$/,
                loader: "file-loader"
            }
        ]
    },
    plugins: [
        new VueLoaderPlugin(),
        new MiniCssExtractPlugin({
            filename:"bundle.css"
        })
    ],
    resolve: {
        alias: {
            'vue$': 'vue/dist/vue.esm.js'
        }
    },
    // devtool:"source-map"
}
