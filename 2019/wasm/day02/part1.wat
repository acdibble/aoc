(module
  (table 3 funcref)
  (elem (i32.const 1) $add $mul)
  (global $pointer (mut i32) (i32.const 0))
  (global $break i32 (i32.const 99))
  (memory (export "memory") 1)
  (func $incrementPointer (result i32)
    global.get $pointer
    i32.const 1
    i32.add
    global.set $pointer
    global.get $pointer
  )
  (func $loadAt (param $index i32) (result i32)
    local.get $index
    i32.const 4
    i32.mul
    i32.load
  )
  (func $loadIndirect (result i32)
    call $incrementPointer
    call $loadAt
    call $loadAt
  )
  (func $storeResult (param $value i32)
    call $incrementPointer
    call $loadAt
    i32.const 4
    i32.mul
    local.get $value
    i32.store
  )
  (func $add
    call $loadIndirect
    call $loadIndirect
    i32.add
    call $storeResult
    call $incrementPointer
    drop
  )
  (func $mul
    call $loadIndirect
    call $loadIndirect
    i32.mul
    call $storeResult
    call $incrementPointer
    drop
  )
  (func $processIntcodes (param $byteLength i32) (result i32)
    i32.const 4
    i32.const 12
    i32.store
    i32.const 8
    i32.const 2
    i32.store

    (block
      (loop
        global.get $pointer
        call $loadAt

        global.get $break
        i32.eq
        (br_if 1)

        global.get $pointer
        call $loadAt
        call_indirect

        ;; sanity check to stay within bounds
        global.get $pointer
        local.get $byteLength
        i32.eq

        (br_if 1)
        (br 0)
      )
    )

    i32.const 0
    i32.load
  )
  (export "processIntcodes" (func $processIntcodes))
)
