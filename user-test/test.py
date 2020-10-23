from polaroid import Image,Gif
import time

# start = time.perf_counter()
# im = Gif("p2.gif")
# l = list()
# for frame in im.iterator():
#     frame.vertical_strips(10)
#     l.append(frame)
#
# by = im.save_bytes(l)
# end = time.perf_counter()
# print(end-start)
# with open("r3.gif","wb") as file:
#     file.write(by)
#im.save()

with open("p1.jpg",'rb') as r:
    byt = r.read()
start = time.perf_counter()
im = Image(byt)
print(im.size)
im.offset_red(30)
im.save("ror.png")
end = time.perf_counter()
# with open('ror.jpg','wb') as w:
#     w.write(ret)

print(end-start)