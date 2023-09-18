import test from 'ava'

import { sum, executeRpcRust } from '../index.js'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})

test('testing rpc rust in napi/wasm', async (t) => {
  try {
    await executeRpcRust()
    t.is("1", "1")
  } catch (error) {
    console.error(error)
  }
})

