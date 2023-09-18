Execute dcl-rpc with napi-rs

```bash
$ npm i -g @napi-rs/cli
```

```bash
$ npm install
```

```bash
$ npm run build
```

Execute it by running the tests:

```bash
$ npm run test
```

Execute it manually:
```bash
$ node 
Welcome to Node.js v18.17.0.
Type ".help" for more information.
> const addon = require('./index.js')
undefined
> addon.executeRpcRust()
//...
```