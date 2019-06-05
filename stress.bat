echo start stress test
FOR /L %%A IN (1,1,%1) DO cargo test | find /i "FAILED"
if not errorlevel 1 (
   echo Tests did not pass
)
mailsend-go -sub "Test"  -smtp smtp.gmail.com -port 587 \
     auth \
      -user ryanmeakins@gmail.com -pass "secret" \
     -from "ryanmeakins@gmail.com" -to  "ryanmeakins@example.com" \
     body \
       -msg "hello, world!"