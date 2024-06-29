# ICFP 2024

~~~
cd cate
cargo build --release
cd ../
mkdir -p bin
cp cate/target/release/cate bin/
cp env.sh.example env.sh

source ../env.sh

bin/cate encode "Hi!"
B)_

bin/cate decode "B)_"
Hi!

bin/cate send "get index"
Sending message: get index
Sending message: S'%4}).$%8
body:  YHello and welcome to the School of the Bound Variable!

Before taking a course, we suggest that you have a look around. You're now looking at the [index]. To practice your communication skills, you can use our [echo] service. Furthermore, to know how you and other students are doing, you can look at the [scoreboard].

Once you are ready, please progress to one of the courses that you are currently enrolled in:

 * [lambdaman]
 * [spaceship]
 * [3d]
 * [efficiency]

After passing some tests, you may be admitted to other courses, so make sure to check this page from time to time. In the meantime, if you want to practice more advanced communication skills, you may also take our [language_test].


~~~