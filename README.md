## notes for this fork:
- updates to rust 2018
- updated deps
- turned polymorphism into enum variants and reduced boxing which gives better performance

- need to measure performance more carefully before giving numbers but should be noticeable
- the original goal of the author was a simple raytracer so optimizing it was easy and just a fun little exercise, there are other versions of it available with multi threading or simd


# Ray tracing in one weekend

This is a straightforward port of the code in
[Ray Tracing In One Weekend](http://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html)
by Peter Shirley.

It's a wonderful book, and you can get it for $2.99 [on amazon](http://www.amazon.com/gp/product/B01B5AODD8/ref=as_li_tl?ie=UTF8&camp=1789&creative=9325&creativeASIN=B01B5AODD8&linkCode=as2&tag=inonwe09-20&linkId=OPNJXXJY2IBCMEGE).

![picture of some nice spheres](pretty.png)
