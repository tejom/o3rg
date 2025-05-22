import o3rg

from time import time
import os

# try:
#     print(o3rg.search("Cargo.toml", "+++\\4+=\0"))
# except ValueError as e:
#     print(e)

# print(o3rg.search("Cargo.toml", "ver"))

start = time()
for root, dirs , files in os.walk("./"):
    #print(root, files)
    #print(dirs)
    for fn in files: 
        #print(f"{root}/{fn}")
        res = o3rg.search(f"{root}/{fn}", "Searcher")
        if res:
            print(res, fn)
end = time()
print(end - start)

start = time()
for r in o3rg.search_dir("./", "Searcher", hidden=True):
      print(r)
end = time()
print(end - start)



# print(o3rg.search_dir("./", "Searcher"))
