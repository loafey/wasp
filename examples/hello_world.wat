(module
    (import  "console"  "log" (func  $log (param  i32  i32))) ;; Import log function
    (import  "js"  "mem" (memory  1)) ;; Import 1 page of memory (54kb)
    
    (data (i32.const 0) "Hello world!")
    
    (func (export  "main")
        i32.const 0  
        i32.const 12  
        call  $log
        )
)