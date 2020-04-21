(module
  (memory (export "memory") 1)
  (func $loadIndex (param $index i32) (result i32)
    local.get $index
    i32.const 4
    i32.mul
    i32.load
  )
  (func $calculateFuel (param $fuel i32) (result i32)
    local.get $fuel
    i32.const 3
    i32.div_u
    i32.const 2
    i32.sub
  )
  (func $accumulate (param $end i32) (result i32)
    (local $index i32)
    (local $output i32)

    i32.const 0
    local.tee $index
    local.set $output

    (block
      (loop
        local.get $index
        call $loadIndex
        call $calculateFuel
        local.get $output
        i32.add
        local.set $output

        local.get $index
        i32.const 1
        i32.add
        local.tee $index
        local.get $end
        i32.eq

        (br_if 1)
        (br 0)
      )
    )
    local.get $output
  )
  (export "accumulate" (func $accumulate))
)
