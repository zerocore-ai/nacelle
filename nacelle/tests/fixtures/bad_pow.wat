(module
    (func (export "bad_pow") (param $value f64) (param $power i32) (result f64)
        (local $result f64)
        (local $i i32)

        (if (i32.eq (local.get $power) (i32.const 0))
            (then (return (f64.const 1.0)))
        )

        (if (i32.lt_s (local.get $power) (i32.const 0))
            (then (return (f64.const 0.0)))
        )

        (local.set $result (f64.const 1.0))
        (local.set $i (i32.const 0))

        (block $break
            (loop $continue
                (br_if $break (i32.ge_u (local.get $i) (local.get $power))) ;; break

                (local.set $result (f64.mul (local.get $result) (local.get $value)))
                (local.set $i (i32.add (local.get $i) (i32.const 1)))

                br $continue ;; continue
            )
        )

        (local.get $result)
    )
)
