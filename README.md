pug-rs
---

*pug.js reimplemented in rust for speed*


### Usage
```
✗ git clone https://github.com/vitali2y/pug-rs && cd pug-rs
✗ cargo b
✗ cat ./assets/{index.pug,head.pug,foot.pug}
//- index.pug
doctype html
html
  include head.pug
  body
    h1 Index
    include foot.pug
//- head.pug
head
  title Head
  script(src='/js/app.js')
//- foot.pug
footer#footer
  p Footer
✗ pug < assets/index.pug > ./index.html && cat ./index.html && open ./index.html
<!DOCTYPE html><html><head><title>Head</title><script src='/js/app.js'></script></head><body><h1>Index</h1><footer id="footer"><p>Footer</p></footer></body></html>
✗
```
