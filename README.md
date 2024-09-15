# Download and run MemFS WebDAV server
Source [here](https://gist.github.com/mickael-kerjean/f2f034bdad5e077edcbfdff649d52d68)
```sh
curl -O https://archive.kerjean.me/public/2022/webdav.bin
chmod a+x webdav.bin 
./webdav.bin 
```
# Example run
```
Initial listing: Ok("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<multistatus xmlns=\"DAV:\">\n <response>\n  <href>/</href>\n </response>\n</multistatus>")
Write 'hello world' to test.txt: Ok("")
Listing: Ok("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<multistatus xmlns=\"DAV:\">\n <response>\n  <href>/test.txt</href>\n </response>\n <response>\n  <href>/</href>\n </response>\n</multistatus>")
Content of test.txt: Ok("hello world")
Overwrite test.txt: Ok("")
Content of test.txt: Ok("goodbye")
Delete test.txt: Ok("")
Final listing: Ok("")
```
