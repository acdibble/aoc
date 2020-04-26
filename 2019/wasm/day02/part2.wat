(module
  (table 5 funcref)
  (elem (i32.const 1) $add $mul $processOpcode $reset)
  (type $takeOneReturnOne (func (param i32) (result i32)))
  (global $pointer (mut i32) (i32.const 0))
  (global $processOpcode i32 (i32.const 3))
  (global $reset i32 (i32.const 4))
  (global $break i32 (i32.const 99))
  (global $sought i32 (i32.const 19690720))
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
  (func $reset (param $byteLength i32) (result i32)
    i32.const 1111111111
    drop
    global.get $pointer
    i32.const 4
    i32.mul
    local.get $byteLength
    global.get $pointer
    i32.add
    i32.const 4
    i32.mul
    i32.load
    i32.store
    call $incrementPointer
    drop
    i32.const 0
  )
  (func $loop (param $func i32) (param $byteLength i32)
    i32.const 0
    global.set $pointer

    (block
      (loop
        local.get $byteLength
        local.get $func
        call_indirect (type $takeOneReturnOne)
        (br_if 1)
        ;; sanity check to stay within bounds
        global.get $pointer
        local.get $byteLength
        i32.eq

        (br_if 1)
        (br 0)
      )
    )
  )
  (func $processOpcode (param i32) (result i32)
    global.get $pointer
    call $loadAt

    global.get $break
    i32.eq
    if
      i32.const 1
      return
    end

    global.get $pointer
    call $loadAt
    call_indirect
    i32.const 0
  )
  (func $processIntcodes (param $byteLength i32) (result i32)
    (local $x i32)
    (local $y i32)
    (block
      (loop
        i32.const 0
        local.set $y
        i32.const 1
        local.get $x
        i32.add
        local.tee $x
        i32.const 99
        i32.eq
        (br_if 1) ;; should never happen



        (loop
          i32.const 4
          local.get $x
          i32.store
          i32.const 1
          local.get $y
          i32.add
          local.tee $y
          i32.const 99
          i32.eq

          (br_if 1)
          i32.const 8
          local.get $y
          i32.store

          global.get $processOpcode
          local.get $byteLength
          call $loop
          i32.const 0
          i32.load
          global.get $sought
          i32.eq

          (br_if 2)
          global.get $reset
          local.get $byteLength
          call $loop
          (br 0)
        )
      )
    )

    i32.const 8
    i32.load
    i32.const 4
    i32.load
    i32.const 100
    i32.mul
    i32.add
  )
  (export "processIntcodes" (func $processIntcodes))
)
