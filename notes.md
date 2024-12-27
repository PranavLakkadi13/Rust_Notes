
-> This command was use learn and run the test in the tests folder for http_test   
-> quick_dev is the name of the async fn that its supposed listen   
-> watch is like the nodemon eqvivalent   
-> -q is quiet   
-> -c stands for clear the screen after the code is re-run   
-> -w stands for watch tests/ (folder)   
-> -x stands for execute when change is executed    
 ``` javascript 
 cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture "
 ```


-> uses the similar above command to run the src folder 
 ``` javascript 
 cargo watch -q -c -w src/ -x run
 ```


