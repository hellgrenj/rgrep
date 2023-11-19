# rgrep

I made a stupid (but fast) version of grep in rust as a learning exercise

``` ➜  temp ls -sh
total 1.8G
1.8G largefile
➜  temp time grep gorilla largefile
Once upon a time in a lush, green bamboo forest, there lived a unique gorilla named Ping. Ping was not like other pandas; he had a deep fascination for programming. His eyes would light up whenever he found old computers or gadgets left by tourists. Ping spent his days learning to code by watching tutorials on abandoned tablets and practicing on a weathered laptop.
Once upon a time in a lush, green bamboo forest, there lived a unique gorilla named Ping. Ping was not like other pandas; he had a deep fascination for programming. His eyes would light up whenever he found old computers or gadgets left by tourists. Ping spent his days learning to code by watching tutorials on abandoned tablets and practicing on a weathered laptop.
Once upon a time in a lush, green bamboo forest, there lived a unique gorilla named Ping. Ping was not like other pandas; he had a deep fascination for programming. His eyes would light up whenever he found old computers or gadgets left by tourists. Ping spent his days learning to code by watching tutorials on abandoned tablets and practicing on a weathered laptop.
Once upon a time in a lush, green bamboo forest, there lived a unique gorilla named Ping. Ping was not like other pandas; he had a deep fascination for programming. His eyes would light up whenever he found old computers or gadgets left by tourists. Ping spent his days learning to code by watching tutorials on abandoned tablets and practicing on a weathered laptop.
grep --color=auto --exclude-dir={.bzr,CVS,.git,.hg,.svn,.idea,.tox} gorilla   0.91s user 0.17s system 99% cpu 1.082 total
➜  temp time rgrep gorilla largefile
Once upon a time in a lush, green bamboo forest, there lived a unique gorilla named Ping. Ping was not like other pandas; he had a deep fascination for programming. His eyes would light up whenever he found old computers or gadgets left by tourists. Ping spent his days learning to code by watching tutorials on abandoned tablets and practicing on a weathered laptop.
Once upon a time in a lush, green bamboo forest, there lived a unique gorilla named Ping. Ping was not like other pandas; he had a deep fascination for programming. His eyes would light up whenever he found old computers or gadgets left by tourists. Ping spent his days learning to code by watching tutorials on abandoned tablets and practicing on a weathered laptop.
Once upon a time in a lush, green bamboo forest, there lived a unique gorilla named Ping. Ping was not like other pandas; he had a deep fascination for programming. His eyes would light up whenever he found old computers or gadgets left by tourists. Ping spent his days learning to code by watching tutorials on abandoned tablets and practicing on a weathered laptop.
Once upon a time in a lush, green bamboo forest, there lived a unique gorilla named Ping. Ping was not like other pandas; he had a deep fascination for programming. His eyes would light up whenever he found old computers or gadgets left by tourists. Ping spent his days learning to code by watching tutorials on abandoned tablets and practicing on a weathered laptop.
rgrep gorilla largefile  0.36s user 0.21s system 99% cpu 0.565 total ``` 

