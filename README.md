# rust-pangenome-graph-bed
 
 - rust pangenome graph bed converted
 - Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version.


 ```
 cargo build 

 ```

 ```
 ╭─gauravsablok@fedora ~/Desktop/rust-pangenome-graph-linear  ‹main*› 
 ╰─➤  ./target/debug/rust-pangenome-graph-bed -h
 Usage: rust-pangenome-graph-bed <GRAPH>

 Arguments:
  <GRAPH>  please provide the path to the graph file

 Options:
  -h, --help     Print help
  -V, --version  Print version  
 
 ```

 ```
 ./target/debug/rust-pangenome-graph-bed ./sample-file/sample-pangenome.gfa
 "MT_human"	0	4001	"MTh0"	0
 "MT_human"	4001	4502	"MTh4001"	0
 "MT_orang"	3426	3927	"MTo3426"	1
 "MT_human"	4502	9505	"MTh4502"	0
 "MT_orang"	8961	9463	"MTo8961"	1
 "MT_human"	9505	13014	"MTh9505"	0
 "MT_human"	13014	13516	"MTh13014"	0
 "MT_human"	13516	16569	"MTh13516"	0
 ```

 Gaurav Sablok
