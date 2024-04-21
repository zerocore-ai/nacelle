(module
    (import "wasi_snapshot_preview1" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))
    (import "wasi_snapshot_preview1" "fd_read" (func $fd_read (param i32 i32 i32 i32) (result i32)))
    (import "wasi_snapshot_preview1" "fd_close" (func $fd_close (param i32) (result i32)))
    (import "wasi_snapshot_preview1" "fd_prestat_get" (func $fd_prestat_get (param i32 i32) (result i32)))
    (import "wasi_snapshot_preview1" "path_open" (func $path_open (param i32 i32 i32 i32 i32 i64 i64 i32 i32) (result i32)))

    (memory (export "memory") 1)

    (global $foo_filename_ptr i32 (i32.const 32))
    (global $bar_filename_ptr i32 (i32.const 48))
    (global $filename_len i32 (i32.const 16))

    (global $opened_fd_ptr i32 (i32.const 128))

    (global $iov_vec_ptr i32 (i32.const 256))
    (global $iov_vec_len i32 (i32.const 1))

    (global $block_0_ptr i32 (i32.const 512))
    (global $block_0_len i32 (i32.const 128))

    (data (i32.const 32) "archives/foo.txt")
    (data (i32.const 48) "archives/bar.txt")

    (func (export "copy_file") (result i32)
        (local $foo_fd i32)
        (local $bar_fd i32)
        (local $preopened_fd i32)
        (local $errno i32)

        ;; Get the first preopened fd
        (call $get_first_preopened_fd)

        ;; Store the fd and errno
        (local.set $errno)
        (local.set $preopened_fd)

        ;; If we got an error, return it
        (if (i32.eq (local.get $preopened_fd) (i32.const -1))
            (then (return (local.get $errno)))
        )

        ;; Open foo.txt
        (call $path_open
            (local.get $preopened_fd) ;; dirfd
            (i32.const 0) ;; lookupflags
            (global.get $foo_filename_ptr) ;; filename
            (global.get $filename_len) ;; filename_len
            (i32.const 0) ;; oflags
            (i64.const 66) ;; rights_base; read | write
            (i64.const 66) ;; rights_inheriting; read | write
            (i32.const 0) ;; fdflags
            (global.get $opened_fd_ptr) ;; opened_fd
        )

        ;; Check for errors
        (local.set $errno)
        (if (i32.ne (local.get $errno) (i32.const 0))
            (then (return (local.get $errno)))
        )

        ;; Store the file descriptor
        (local.set $foo_fd (i32.load (global.get $opened_fd_ptr)))

        ;; Open bar.txt
        (call $path_open
            (local.get $preopened_fd) ;; dirfd
            (i32.const 0) ;; lookupflags
            (global.get $bar_filename_ptr) ;; filename
            (global.get $filename_len) ;; filename_len
            (i32.const 1) ;; oflags; create
            (i64.const 66) ;; rights_base; read | write
            (i64.const 66) ;; rights_inheriting; read | write
            (i32.const 0) ;; fdflags
            (global.get $opened_fd_ptr) ;; opened_fd
        )

        ;; Check for errors
        (local.set $errno)
        (if (i32.ne (local.get $errno) (i32.const 0))
            (then (return (local.get $errno)))
        )

        ;; Store the file descriptor
        (local.set $bar_fd (i32.load (global.get $opened_fd_ptr)))

        ;; Set the iovs
        (i32.store (global.get $iov_vec_ptr) (global.get $block_0_ptr)) ;; iovs[0].buf
        (i32.store offset=4 (global.get $iov_vec_ptr) (global.get $block_0_len)) ;; iovs[0].buf_len

        ;; Read from foo.txt
        (call $fd_read
            (local.get $foo_fd) ;; fd
            (global.get $iov_vec_ptr) ;; iovs
            (global.get $iov_vec_len) ;; iovs_len
            (i32.const 0) ;; nread
        )

        ;; Check for errors
        (local.set $errno)
        (if (i32.ne (local.get $errno) (i32.const 0))
            (then (return (local.get $errno)))
        )

        ;; Set the iovs
        (i32.store (global.get $iov_vec_ptr) (global.get $block_0_ptr)) ;; iovs[0].buf
        (i32.store offset=4 (global.get $iov_vec_ptr) (call $get_block_0_string_length)) ;; iovs[0].buf_len

        ;; Write to bar.txt
        (call $fd_write
            (local.get $bar_fd) ;; fd
            (global.get $iov_vec_ptr) ;; iovs
            (global.get $iov_vec_len) ;; iovs_len
            (i32.const 0) ;; nwritten
        )

        ;; Check for errors
        (local.set $errno)
        (if (i32.ne (local.get $errno) (i32.const 0))
            (then (return (local.get $errno)))
        )

        ;; Otherwise, success.
        (i32.const 0)
    )

    (func $get_block_0_string_length (result i32)
        ;; loop and count non-null bytes starting at block_0_ptr
        (local $ptr i32)
        (local $len i32)

        (local.set $ptr (global.get $block_0_ptr))
        (local.set $len (i32.const 0))

        (block $break
            (loop $continue
                (if (i32.eqz (i32.load (local.get $ptr)))
                    (then (br $break))
                )

                (local.set $len (i32.add (local.get $len) (i32.const 1)))
                (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))

                (br $continue)
            )
        )

        (return (local.get $len))
    )

    (func $get_first_preopened_fd (export "get_first_preopened_fd") (result i32 i32) ;; (fd, errno)
        ;; loop and increment fd until we find the preopened fd. If we get ERRNO_BADF, then we know we've  reached the end of the preopened fds
        (local $fd i32)
        (local $errno i32)

        ;; Start with lowest fd after stdin, stdout, stderr
        (local.set $fd (i32.const 3))

        (block $break
            (loop $continue
                (call $fd_prestat_get
                    (local.get $fd) ;; fd
                    (i32.const 0) ;; buf
                ) ;; returns errno

                ;; Store the errno
                (local.set $errno)

                ;; Break after we get the first good fd
                (br_if $break (i32.eqz (local.get $errno)))

                ;; If we get ERRNO_BADF, return it.
                (if (i32.eq (local.get $errno) (i32.const 8))
                    (then (return (i32.const -1) (local.get $errno)))
                )

                ;; Otherwise, increment fd and continue
                (local.set $fd (i32.add (local.get $fd) (i32.const 1)))

                (br $continue)
            )
        )

        (return (local.get $fd) (local.get $errno))
    )
)
