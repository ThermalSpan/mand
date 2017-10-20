# mand

### What

mand is a simple mandelbrot set viewer. It opens a window that has some (hopefully) intuitive camera controls to move the view around and explore.

### How

This was the project I used to start exploring [glium](https://crates.io/crates/glium). I let it stagnate for almost a year, and decided to revitalize it with all the rust technique I have learned since. Also, since it took a while to get it to compiler again, the toolchain I'm currently using is:

    active toolchain
    ----------------

    nightly-x86_64-apple-darwin (default)
    rustc 1.22.0-nightly (7778906be 2017-10-14)

## Future Work

* Get some real error handling in here with error-chain
* Finish the work with the plane selectors so I can control color and such
* Learn about [smooth iteration count](http://iquilezles.org/www/articles/mset_smooth/mset_smooth.htm) and apply those techniques here

