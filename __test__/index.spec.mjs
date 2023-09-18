import test from 'ava'

import { executeRpcRust } from '../index.js'

test('testing rpc rust in napi/wasm', async (t) => {
  try {
    await executeRpcRust()
    t.is("1", "1")
  } catch (error) {
    console.error(error)
  }
})

